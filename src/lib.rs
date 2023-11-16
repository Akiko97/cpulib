#![allow(rustdoc::invalid_rust_codeblocks)]
//! A CPU emulator library designed to simulate the CPU context, with a specific focus on SIMD operations.
//!
//! # Usage
//!
//! ```rust
//! use cpulib::{ CPU, Utilities, u256, u512, VecRegName, GPRName, FLAGSName, IPName };
//!
//! let mut cpu = CPU::default();
//!
//! cpu.registers.set_bit(VecRegName::XMM, 0, 127, true);
//! println!("{:?}", cpu.registers.get_bit(VecRegName::XMM, 0, 127));
//! ```

extern crate primitive_types;
pub use primitive_types::U256 as u256;
pub use primitive_types::U512 as u512;

mod registers;
mod memory;
mod utilities;
mod instructions;

pub use registers::Registers;
pub use registers::VecRegName;
pub use registers::GPRName;
pub use registers::FLAGSName;
pub use registers::IPName;

pub use memory::Memory;

pub use utilities::Utilities;

/// Represents the CPU context in the emulator.
///
/// Contains registers and memory components necessary for CPU operations.
/// This structure is central to performing various CPU operations and simulations.
///
/// # Fields
/// * `registers` - Stores the CPU registers, including general-purpose, vector, and system registers.
/// * `memory` - Represents the memory accessible by the CPU, allowing read and write operations.
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
}

impl CPU {
    /// Creates a new CPU context with a specified memory base address.
    ///
    /// Initializes the CPU with a set of registers and a memory model starting from the given base address.
    /// This allows for a customizable memory layout.
    ///
    /// # Arguments
    /// * `base` - A `usize` representing the base address for the CPU's memory.
    ///
    /// # Returns
    /// Returns a new `CPU` instance with initialized registers and memory.
    pub fn new(base: usize) -> Self {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(base)
        }
    }
}

impl Default for CPU {
    /// Creates a new CPU context with a default memory base address.
    ///
    /// Initializes the CPU with a standard set of registers and a default memory model.
    /// The default memory base address is typically set to a standard value for convenience.
    ///
    /// # Returns
    /// Returns a new `CPU` instance with default settings.
    fn default() -> Self {
        CPU::new(0x00400000usize)
    }
}

/// Contains unit tests for the CPU emulator.
///
/// These tests cover various aspects of the CPU's functionality, including register operations,
/// memory reads and writes, and handling of different data types. They ensure that the CPU
/// emulator behaves as expected in a range of scenarios.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut cpu = CPU::default();
        // test set/get bit
        cpu.registers.set_bit(VecRegName::XMM, 0, 127, true);
        if let Some(result) = cpu.registers.get_bit(VecRegName::XMM, 0, 127) {
            assert_eq!(result, true);
        }
        if let Some(result) = cpu.registers.get_bit(VecRegName::YMM, 0, 127) {
            assert_eq!(result, true);
        }
        if let Some(result) = cpu.registers.get_bit(VecRegName::ZMM, 0, 127) {
            assert_eq!(result, true);
        }
        cpu.registers.set_bit(VecRegName::YMM, 0, 255, true);
        if let Some(result) = cpu.registers.get_bit(VecRegName::YMM, 0, 255) {
            assert_eq!(result, true);
        }
        cpu.registers.set_bit(VecRegName::ZMM, 0, 511, true);
        if let Some(result) = cpu.registers.get_bit(VecRegName::ZMM, 0, 511) {
            assert_eq!(result, true);
        }
        // test get sections
        cpu.registers.set_bit(VecRegName::ZMM, 1, 0, true);
        cpu.registers.set_bit(VecRegName::ZMM, 1, 511, true);
        if let Some(result) = cpu.registers.get_by_sections::<u64>(VecRegName::ZMM, 1) {
            assert_eq!(result.len(), 8);
            assert_eq!(result[0], 1);
            assert_eq!(result[7], 9223372036854775808);
        }
        // test set sections
        assert_eq!(cpu.registers.set_by_sections(VecRegName::XMM, 2, vec![2147483648u32, 2147483648u32, 2147483648u32, 2147483648u32]), true);
        if let Some(result) = cpu.registers.get_by_sections::<u32>(VecRegName::XMM, 2) {
            assert_eq!(result.len(), 4);
            assert_eq!(result[0], 2147483648u32);
            assert_eq!(result[1], 2147483648u32);
            assert_eq!(result[2], 2147483648u32);
            assert_eq!(result[3], 2147483648u32);
        }
        // test GPR
        cpu.registers.set_gpr_value(GPRName::RAX, 18446744073709486080u64);
        assert_eq!(cpu.registers.get_gpr_value(GPRName::RAX), 18446744073709486080u64);
        cpu.registers.set_gpr_value(GPRName::AL, 255u64);
        assert_eq!(cpu.registers.get_gpr_value(GPRName::RAX), 18446744073709486335u64);
        cpu.registers.set_gpr_value(GPRName::EAX, 65535u64);
        assert_eq!(cpu.registers.get_gpr_value(GPRName::RAX), 65535u64);
        // test type u256 & u512
        assert_eq!(cpu.registers.set_by_sections(VecRegName::ZMM, 3, vec![u256::from(1), u256::from(2)]), true);
        if let Some(result) = cpu.registers.get_by_sections::<u256>(VecRegName::ZMM, 3) {
            assert_eq!(result.len(), 2);
            assert_eq!(result[0], u256::from(1usize));
            assert_eq!(result[1], u256::from(2usize));
        }
        assert_eq!(cpu.registers.set_by_sections(VecRegName::ZMM, 5, vec![u512::from(1)]), true);
        if let Some(result) = cpu.registers.get_by_sections::<u512>(VecRegName::ZMM, 5) {
            assert_eq!(result.len(), 1);
            assert_eq!(result[0], u512::from(1usize));
        }
        // test float values
        assert_eq!(cpu.registers.set_by_sections(VecRegName::XMM, 6, Utilities::f32vec_to_u32vec(vec![1.0f32, 2.0f32, 3.0f32, 4.0f32])), true);
        if let Some(u32vec) = cpu.registers.get_by_sections::<u32>(VecRegName::XMM, 6) {
            let result = Utilities::u32vec_to_f32vec(u32vec);
            assert_eq!(result.len(), 4);
            assert_eq!(result[0], 1.0f32);
            assert_eq!(result[1], 2.0f32);
            assert_eq!(result[2], 3.0f32);
            assert_eq!(result[3], 4.0f32);
        }
        assert_eq!(cpu.registers.set_by_sections(VecRegName::XMM, 7, Utilities::f64vec_to_u64vec(vec![1.0f64, 2.0f64])), true);
        if let Some(u64vec) = cpu.registers.get_by_sections::<u64>(VecRegName::XMM, 7) {
            let result = Utilities::u64vec_to_f64vec(u64vec);
            assert_eq!(result.len(), 2);
            assert_eq!(result[0], 1.0f64);
            assert_eq!(result[1], 2.0f64);
        }
        // test selector
        cpu.registers.set_by_sections::<u32>(VecRegName::XMM, 15, vec![
            0x12345678u32, 0x12345678u32, 0x12345678u32, 0x12345678u32,
        ]);
        cpu.registers.set_by_selector::<u32>(VecRegName::XMM, 15, "[31:0]", 0x00000000u32);
        if let Some(result) = cpu.registers.get_by_sections::<u32>(VecRegName::XMM, 15) {
            assert_eq!(result[0], 0u32);
            assert_eq!(result[1], 0x12345678u32);
            assert_eq!(result[2], 0x12345678u32);
            assert_eq!(result[3], 0x12345678u32);
        }
        cpu.registers.set_by_selector::<u32>(VecRegName::XMM, 15, "[MAX:64]", 0x00000000u32);
        if let Some(result) = cpu.registers.get_by_sections::<u32>(VecRegName::XMM, 15) {
            assert_eq!(result[0], 0u32);
            assert_eq!(result[1], 0x12345678u32);
            assert_eq!(result[2], 0u32);
            assert_eq!(result[3], 0u32);
        }
        // test memory
        assert_eq!(cpu.memory.read::<u8>(0x00400000), 0);
        cpu.memory.write::<u8>(0x00400000, 0x12);
        assert_eq!(cpu.memory.read::<u8>(0x00400000), 0x12);
        cpu.memory.write::<u16>(0x00400000, 0x1234);
        assert_eq!(cpu.memory.read::<u16>(0x00400000), 0x1234);
        cpu.memory.write::<u32>(0x00400000, 0x12345678);
        assert_eq!(cpu.memory.read::<u32>(0x00400000), 0x12345678);
        cpu.memory.write::<u64>(0x00400000, 0x1234567887654321);
        assert_eq!(cpu.memory.read::<u64>(0x00400000), 0x1234567887654321);
        cpu.memory.write::<u128>(0x00400000, 0x12345678876543211234567887654321);
        assert_eq!(cpu.memory.read::<u128>(0x00400000), 0x12345678876543211234567887654321);
        cpu.memory.write::<u256>(0x00400000, u256::from(0x12345678876543211234567887654321u128));
        assert_eq!(cpu.memory.read::<u256>(0x00400000), u256::from(0x12345678876543211234567887654321u128));
        cpu.memory.write::<u512>(0x00400000, u512::from(0x12345678876543211234567887654321u128));
        assert_eq!(cpu.memory.read::<u512>(0x00400000), u512::from(0x12345678876543211234567887654321u128));
        cpu.memory.write_vec::<u64>(0x00400000, vec![
            0, 1, 2, 3, 4, 5, 6, 7,
        ]);
        let result = cpu.memory.read_vec::<u32>(0x00400000, 16);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 1);
        assert_eq!(result[3], 0);
        assert_eq!(result[4], 2);
        assert_eq!(result[5], 0);
        assert_eq!(result[6], 3);
        assert_eq!(result[7], 0);
        assert_eq!(result[8], 4);
        assert_eq!(result[9], 0);
        assert_eq!(result[10], 5);
        assert_eq!(result[11], 0);
        assert_eq!(result[12], 6);
        assert_eq!(result[13], 0);
        assert_eq!(result[14], 7);
        assert_eq!(result[15], 0);
    }
}
