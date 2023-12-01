extern crate bit_vec;
extern crate regex;

use std::fmt::{Display, Formatter};
use bit_vec::BitVec;
use regex::Regex;

// trait alias and enum
/// A trait alias representing a collection of traits necessary for section compatibility.
///
/// This trait ensures the implementing type supports basic arithmetic and bitwise operations
/// required for section manipulation in SIMD registers.
pub trait SectionCompatible:
    From<u8> + Copy + Eq +
    std::ops::Shl<usize, Output = Self> + std::ops::Shr<usize, Output = Self> +
    std::ops::BitOr<Output = Self> + std::ops::BitAnd<Output = Self>
{}

impl<T:
    From<u8> + Copy + Eq +
    std::ops::Shl<usize, Output = T> + std::ops::Shr<usize, Output = T> +
    std::ops::BitOr<Output = T> + std::ops::BitAnd<Output = T>> SectionCompatible for T
{}

/// An enumeration of SIMD (Single Instruction, Multiple Data) vector register names.
///
/// This enum represents various SIMD registers, such as XMM, YMM, and ZMM, which are
/// commonly used in advanced processor features for parallel data processing.
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum VecRegName {
    XMM, YMM, ZMM
}

/// Implements the `Display` trait for `VecRegName`.
///
/// This implementation allows for the human-readable representation of the SIMD vector
/// register names. Each variant of the `VecRegName` is formatted into its corresponding
/// register name string.
impl Display for VecRegName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            VecRegName::XMM => "XMM",
            VecRegName::YMM => "YMM",
            VecRegName::ZMM => "ZMM",
        })
    }
}

/// An enumeration representing General Purpose Register (GPR) names.
///
/// This enum includes register names for various sizes: 64-bit (RAX, RBX, ...),
/// 32-bit (EAX, EBX, ...), 16-bit (AX, BX, ...), and 8-bit (AH, AL, ...).
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum GPRName {
    // 64-bit registers
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP,
    R8, R9, R10, R11, R12, R13, R14, R15,
    // 32-bit registers
    EAX, EBX, ECX, EDX, ESI, EDI, EBP, ESP,
    R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D,
    // 16-bit registers
    AX, BX, CX, DX, SI, DI, BP, SP,
    R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W,
    // 8-bit registers
    AH, BH, CH, DH, AL, BL, CL, DL, SIL, DIL, BPL, SPL,
    R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B
}

/// Implements the `Display` trait for `GPRName`.
///
/// This implementation allows for the human-readable representation of the General Purpose
/// Register names. Each variant of the `GPRName` is formatted into its corresponding
/// register name string.
impl Display for GPRName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            GPRName::RAX => "RAX",
            GPRName::RBX => "RBX",
            GPRName::RCX => "RCX",
            GPRName::RDX => "RDX",
            GPRName::RSI => "RSI",
            GPRName::RDI => "RDI",
            GPRName::RBP => "RBP",
            GPRName::RSP => "RSP",
            GPRName::R8 => "R8",
            GPRName::R9 => "R9",
            GPRName::R10 => "R10",
            GPRName::R11 => "R11",
            GPRName::R12 => "R12",
            GPRName::R13 => "R13",
            GPRName::R14 => "R14",
            GPRName::R15 => "R15",
            GPRName::EAX => "EAX",
            GPRName::EBX => "EBX",
            GPRName::ECX => "ECX",
            GPRName::EDX => "EDX",
            GPRName::ESI => "ESI",
            GPRName::EDI => "EDI",
            GPRName::EBP => "EBP",
            GPRName::ESP => "ESP",
            GPRName::R8D => "R8D",
            GPRName::R9D => "R9D",
            GPRName::R10D => "R10D",
            GPRName::R11D => "R11D",
            GPRName::R12D => "R12D",
            GPRName::R13D => "R13D",
            GPRName::R14D => "R14D",
            GPRName::R15D => "R15D",
            GPRName::AX => "AX",
            GPRName::BX => "BX",
            GPRName::CX => "CX",
            GPRName::DX => "DX",
            GPRName::SI => "SI",
            GPRName::DI => "DI",
            GPRName::BP => "BP",
            GPRName::SP => "SP",
            GPRName::R8W => "R8W",
            GPRName::R9W => "R9W",
            GPRName::R10W => "R10W",
            GPRName::R11W => "R11W",
            GPRName::R12W => "R12W",
            GPRName::R13W => "R13W",
            GPRName::R14W => "R14W",
            GPRName::R15W => "R15W",
            GPRName::AH => "AH",
            GPRName::BH => "BH",
            GPRName::CH => "CH",
            GPRName::DH => "DH",
            GPRName::AL => "AL",
            GPRName::BL => "BL",
            GPRName::CL => "CL",
            GPRName::DL => "DL",
            GPRName::SIL => "SIL",
            GPRName::DIL => "DIL",
            GPRName::BPL => "BPL",
            GPRName::SPL => "SPL",
            GPRName::R8B => "R8B",
            GPRName::R9B => "R9B",
            GPRName::R10B => "R10B",
            GPRName::R11B => "R11B",
            GPRName::R12B => "R12B",
            GPRName::R13B => "R13B",
            GPRName::R14B => "R14B",
            GPRName::R15B => "R15B",
        })
    }
}

/// An enumeration of flag register names for different bit sizes.
///
/// Includes RFLAGS for 64-bit, EFLAGS for 32-bit, and FLAGS for 16-bit registers.
pub enum FLAGSName {
    // 64-bit registers
    RFLAGS,
    // 32-bit registers
    EFLAGS,
    // 16-bit registers
    FLAGS
}

/// An enumeration of Instruction Pointer register names for various sizes.
///
/// This enum includes RIP for 64-bit, EIP for 32-bit, and IP for 16-bit registers.
pub enum IPName {
    // 64-bit registers
    RIP,
    // 32-bit registers
    EIP,
    // 16-bit registers
    IP
}

/// Extracts two usize values from a string formatted as "[value1:value2]".
///
/// This function uses regular expressions to parse a string and extract two numerical
/// values, returning them as a tuple. It supports the special case of "MAX" representing
/// a maximum value.
///
/// # Arguments
/// * `s` - The string to parse.
///
/// # Returns
/// An `Option<(usize, usize)>` representing the extracted values, or `None` if parsing fails.
fn extract_values(s: &str) -> Option<(usize, usize)> {
    let re = Regex::new(r"\[(.*?):(.*?)\]").unwrap();
    re.captures(s).map(|cap| {
        let a_str = cap.get(1).map_or("", |m| m.as_str());
        let b_str = cap.get(2).map_or("", |m| m.as_str());
        let a_value = if a_str == "MAX" { 511 } else { a_str.parse::<usize>().unwrap_or(0) };
        let b_value = if b_str == "MAX" { 511 } else { b_str.parse::<usize>().unwrap_or(0) };
        (a_value, b_value)
    })
}

/// Represents a SIMD register with a dynamic bit vector.
///
/// This struct manages a SIMD register's state using a bit vector, providing methods
/// to set and get individual bits, clear the register, and manipulate register sections.
struct SIMDRegister {
    bits: BitVec,
}

/// Represents a General Purpose Register (GPR) with a 64-bit value.
///
/// This struct encapsulates a 64-bit GPR, providing methods to set and get its value.
struct GPR {
    value: u64,
}

/// Represents a collection of registers within a simulated CPU architecture.
///
/// This struct includes SIMD registers, general-purpose registers (GPRs), flag registers,
/// and instruction pointers, along with methods to manipulate these registers.
pub struct Registers {
    simd_registers: [SIMDRegister; 16],
    gpr: [GPR; 16],
    rflags: u64,
    rip: u64,
}

impl SIMDRegister {
    /// Creates a new SIMDRegister with a specified size.
    ///
    /// Initializes a SIMD register with a given number of bits, all set to false.
    ///
    /// # Arguments
    /// * `size` - The number of bits in the SIMD register.
    fn new(size: usize) -> Self {
        SIMDRegister {
            bits: BitVec::from_elem(size, false),
        }
    }

    /// Sets the value of a specific bit in the SIMD register.
    ///
    /// # Arguments
    /// * `position` - The index of the bit to set.
    /// * `value` - The boolean value to set the bit to.
    fn set_bit(&mut self, position: usize, value: bool) {
        self.bits.set(position, value);
    }

    /// Gets the value of a specific bit in the SIMD register.
    ///
    /// # Arguments
    /// * `position` - The index of the bit to get.
    ///
    /// # Returns
    /// The boolean value of the bit at the specified position.
    fn get_bit(&self, position: usize) -> bool {
        self.bits[position]
    }

    /// Clears all bits in the SIMD register, setting them to false.
    fn clear(&mut self) {
        for i in 0..self.bits.len() {
            self.set_bit(i, false);
        }
    }

    /// Retrieves the contents of the SIMD register as a vector of section-compatible types.
    ///
    /// # Type Parameters
    /// `T` - A type that is compatible with section operations.
    ///
    /// # Returns
    /// A vector of `T` elements representing sections of the register.
    fn get_sections<T: SectionCompatible>(&self) -> Vec<T> {
        let mut sections = Vec::new();
        let type_bits = std::mem::size_of::<T>() * 8;
        for i in (0..self.bits.len()).step_by(type_bits) {
            let mut section_value: T = T::from(0u8);
            for j in 0..type_bits {
                if i + j >= self.bits.len() {
                    break;
                }
                if self.bits[i + j] {
                    section_value = section_value | (T::from(1u8) << j);
                }
            }
            sections.push(section_value);
        }
        sections
    }

    /// Sets the contents of the SIMD register using a vector of section-compatible types.
    ///
    /// # Type Parameters
    /// `T` - A type that is compatible with section operations.
    ///
    /// # Arguments
    /// * `sections` - A vector of `T` elements to set in the register.
    ///
    /// # Returns
    /// `true` if the operation was successful, `false` otherwise.
    fn set_by_sections<T: SectionCompatible>(&mut self, sections: Vec<T>) -> bool {
        let type_bits = std::mem::size_of::<T>() * 8;
        if type_bits * sections.len() != self.bits.len() {
            return false;
        }
        let mut i = 0;
        for section in &sections {
            for j in 0..type_bits {
                if i + j >= self.bits.len() {
                    break;
                }
                if (*section >> j) & T::from(1u8) == T::from(1u8) {
                    self.set_bit(i + j, true);
                }
            }
            i += type_bits;
        }
        true
    }

    /// Gets a value from the SIMD register from a specified range of indices.
    ///
    /// # Type Parameters
    /// `T` - The type of the value to be retrieved.
    ///
    /// # Arguments
    /// * `start_index` - The starting index of the range.
    /// * `end_index` - The ending index of the range.
    ///
    /// # Returns
    /// A `T` value representing the bits from the specified range.
    fn get_by_index<T: SectionCompatible>(&self, start_index: usize, end_index: usize) -> T {
        let size = end_index - start_index + 1;
        let type_bits = std::mem::size_of::<T>() * 8;
        if type_bits < size {
            panic!("Invalid T size for getting value from {} to {}", start_index, end_index);
        }
        let mut value: T = T::from(0u8);
        for i in start_index..=end_index {
            if i >= self.bits.len() {
                break;
            }
            if self.bits[i] {
                value = value | (T::from(1u8) << (i - start_index));
            }
        }
        value
    }

    /// Sets a range of bits in the SIMD register to the value of a section-compatible type.
    ///
    /// # Type Parameters
    /// `T` - The type of the value to be set.
    ///
    /// # Arguments
    /// * `start_index` - The starting index of the range.
    /// * `end_index` - The ending index of the range.
    /// * `value` - The value to set in the specified range.
    fn set_by_index<T: SectionCompatible>(&mut self, start_index: usize, end_index: usize, value: T) {
        let type_bits = std::mem::size_of::<T>() * 8;
        for i in start_index..=end_index {
            if i >= self.bits.len() {
                break;
            }
            if i - start_index >= type_bits {
                self.set_bit(i, false);
            } else if (value >> (i - start_index)) & T::from(1u8) == T::from(1u8) {
                self.set_bit(i, true);
            } else {
                self.set_bit(i, false);
            }
        }
    }
}

impl GPR {
    /// Creates a new General Purpose Register (GPR) initialized to 0.
    fn new() -> Self {
        GPR {
            value: 0,
        }
    }

    /// Sets the value of the GPR.
    ///
    /// # Arguments
    /// * `val` - The 64-bit value to set the GPR to.
    fn set_value(&mut self, val: u64) {
        self.value = val;
    }

    /// Gets the current value of the GPR.
    ///
    /// # Returns
    /// The 64-bit value of the GPR.
    fn get_value(&self) -> u64 {
        self.value
    }
}

impl Clone for GPR {
    fn clone(&self) -> Self {
        GPR {
            value: self.value
        }
    }
}

impl Copy for GPR {}

macro_rules! register_set {
    ($self:ident; $reg_name:expr; $value:expr; $( $r64:ident, $r32:ident, $r16:ident, $r8_l:ident, $r8_h:ident ),*; $( $r64_:ident, $r32_:ident, $r16_:ident, $r8_:ident ),* ) => {
        match $reg_name {
            $(
                GPRName::$r32 => $self.gpr[GPRName::$r64 as usize].value = $value & 0x00000000_FFFFFFFF,
                GPRName::$r16 => $self.gpr[GPRName::$r64 as usize].value = ($self.gpr[GPRName::$r64 as usize].value & 0xFFFFFFFF_FFFF0000) | ($value & 0x00000000_0000FFFF),
                GPRName::$r8_l => $self.gpr[GPRName::$r64 as usize].value = ($self.gpr[GPRName::$r64 as usize].value & 0xFFFFFFFF_FFFFFF00) | ($value & 0x00000000_000000FF),
                GPRName::$r8_h => $self.gpr[GPRName::$r64 as usize].value = ($self.gpr[GPRName::$r64 as usize].value & 0xFFFFFFFF_FFFF00FF) | (($value << 8) & 0x00000000_0000FF00),
            )*
            $(
                GPRName::$r32_ => $self.gpr[GPRName::$r64_ as usize].value = $value & 0x00000000_FFFFFFFF,
                GPRName::$r16_ => $self.gpr[GPRName::$r64_ as usize].value = ($self.gpr[GPRName::$r64_ as usize].value & 0xFFFFFFFF_FFFF0000) | ($value & 0x00000000_0000FFFF),
                GPRName::$r8_ => $self.gpr[GPRName::$r64_ as usize].value = ($self.gpr[GPRName::$r64_ as usize].value & 0xFFFFFFFF_FFFFFF00) | ($value & 0x00000000_000000FF),
            )*
            _ => $self.gpr[$reg_name as usize].set_value($value),
        }
    }
}

macro_rules! register_get {
    ($self:ident; $reg_name:expr; $( $r64:ident, $r32:ident, $r16:ident, $r8_l:ident, $r8_h:ident ),*; $( $r64_:ident, $r32_:ident, $r16_:ident, $r8_:ident ),* ) => {
        match $reg_name {
            $(
                GPRName::$r32 => $self.gpr[GPRName::$r64 as usize].value & 0x00000000_FFFFFFFF,
                GPRName::$r16 => $self.gpr[GPRName::$r64 as usize].value & 0x00000000_0000FFFF,
                GPRName::$r8_l => $self.gpr[GPRName::$r64 as usize].value & 0x00000000_000000FF,
                GPRName::$r8_h => ($self.gpr[GPRName::$r64 as usize].value & 0x00000000_0000FF00) >> 8,
            )*
            $(
                GPRName::$r32_ => $self.gpr[GPRName::$r64_ as usize].value & 0x00000000_FFFFFFFF,
                GPRName::$r16_ => $self.gpr[GPRName::$r64_ as usize].value & 0x00000000_0000FFFF,
                GPRName::$r8_ => $self.gpr[GPRName::$r64_ as usize].value & 0x00000000_000000FF,
            )*
            _ => $self.gpr[$reg_name as usize].get_value(),
        }
    }
}

impl Registers {
    /// Creates a new Registers struct with initialized values.
    ///
    /// Initializes SIMD registers, GPRs, flag registers, and instruction pointers.
    pub fn new() -> Self {
        Registers {
            simd_registers: [
                SIMDRegister::new(512), SIMDRegister::new(512),
                SIMDRegister::new(512), SIMDRegister::new(512),
                SIMDRegister::new(512), SIMDRegister::new(512),
                SIMDRegister::new(512), SIMDRegister::new(512),
                SIMDRegister::new(512), SIMDRegister::new(512),
                SIMDRegister::new(512), SIMDRegister::new(512),
                SIMDRegister::new(512), SIMDRegister::new(512),
                SIMDRegister::new(512), SIMDRegister::new(512),
            ],
            gpr: [
                GPR::new(); 16
            ],
            rflags: 0u64,
            rip: 0u64,
        }
    }

    /// Sets a specific bit in a specified SIMD register.
    ///
    /// # Arguments
    /// * `reg_type` - The type of SIMD register to operate on.
    /// * `reg_index` - The index of the register.
    /// * `bit_position` - The position of the bit to set.
    /// * `value` - The value to set the bit to.
    pub fn set_bit(&mut self, reg_type: VecRegName, reg_index: usize, bit_position: usize, value: bool) {
        match reg_type {
            VecRegName::XMM if bit_position < 128 => {
                self.simd_registers[reg_index].set_bit(bit_position, value);
            }
            VecRegName::YMM if bit_position < 256 => {
                self.simd_registers[reg_index].set_bit(bit_position, value);
            }
            VecRegName::ZMM if bit_position < 512 => {
                self.simd_registers[reg_index].set_bit(bit_position, value);
            }
            _ => eprintln!("Invalid register type or bit position"),
        }
    }

    /// Gets the value of a specific bit in a specified SIMD register.
    ///
    /// # Arguments
    /// * `reg_type` - The type of SIMD register to operate on.
    /// * `reg_index` - The index of the register.
    /// * `bit_position` - The position of the bit to retrieve.
    ///
    /// # Returns
    /// The value of the bit at the specified position, or `None` if invalid.
    pub fn get_bit(&self, reg_type: VecRegName, reg_index: usize, bit_position: usize) -> Option<bool> {
        match reg_type {
            VecRegName::XMM if bit_position < 128 => {
                Some(self.simd_registers[reg_index].get_bit(bit_position))
            }
            VecRegName::YMM if bit_position < 256 => {
                Some(self.simd_registers[reg_index].get_bit(bit_position))
            }
            VecRegName::ZMM if bit_position < 512 => {
                Some(self.simd_registers[reg_index].get_bit(bit_position))
            }
            _ => None,
        }
    }

    /// Clears all bits in a specified SIMD register.
    ///
    /// # Arguments
    /// * `reg_index` - The index of the register to clear.
    pub fn clear(&mut self, reg_index: usize) {
        self.simd_registers[reg_index].clear();
    }

    /// Retrieves sections of a specified SIMD register as a vector of a specific type.
    ///
    /// # Type Parameters
    /// `T` - The type of the sections to be retrieved.
    ///
    /// # Arguments
    /// * `reg_type` - The type of SIMD register to operate on.
    /// * `reg_index` - The index of the register.
    ///
    /// # Returns
    /// A vector of `T` elements representing sections of the register, or `None` if invalid.
    pub fn get_by_sections<T: SectionCompatible>(&self, reg_type: VecRegName, reg_index: usize) -> Option<Vec<T>> {
        let sections: Vec<T> = self.simd_registers[reg_index].get_sections();
        match reg_type {
            VecRegName::XMM => {
                let n = 128 / (std::mem::size_of::<T>() * 8);
                let slice = &sections[..n];
                Some(slice.to_vec())
            }
            VecRegName::YMM => {
                let n = 256 / (std::mem::size_of::<T>() * 8);
                let slice = &sections[..n];
                Some(slice.to_vec())
            }
            VecRegName::ZMM => {
                let n = 512 / (std::mem::size_of::<T>() * 8);
                let slice = &sections[..n];
                Some(slice.to_vec())
            }
        }
    }

    /// Sets sections of a specified SIMD register using a vector of a specific type.
    ///
    /// # Type Parameters
    /// `T` - The type of the sections to be set.
    ///
    /// # Arguments
    /// * `reg_type` - The type of SIMD register to operate on.
    /// * `reg_index` - The index of the register.
    /// * `sections` - The vector of `T` elements to set in the register.
    ///
    /// # Returns
    /// `true` if the operation was successful, `false` otherwise.
    pub fn set_by_sections<T: SectionCompatible>(&mut self, reg_type: VecRegName, reg_index: usize, sections: Vec<T>) -> bool {
        let type_bits = std::mem::size_of::<T>() * 8;
        let register_bits = type_bits * sections.len();
        let fill_sections = (512 - register_bits) / type_bits;
        match reg_type {
            VecRegName::XMM => {
                if register_bits != 128 {
                    return false;
                }
                let mut fill = sections;
                fill.extend(std::iter::repeat(T::from(0u8)).take(fill_sections));
                self.simd_registers[reg_index].set_by_sections(fill);
                true
            }
            VecRegName::YMM => {
                if register_bits != 256 {
                    return false;
                }
                let mut fill = sections;
                fill.extend(std::iter::repeat(T::from(0u8)).take(fill_sections));
                self.simd_registers[reg_index].set_by_sections(fill);
                true
            }
            VecRegName::ZMM => {
                if register_bits != 512 {
                    return false;
                }
                let mut fill = sections;
                fill.extend(std::iter::repeat(T::from(0u8)).take(fill_sections));
                self.simd_registers[reg_index].set_by_sections(fill);
                true
            }
        }
    }

    /// Retrieves a value from a specified SIMD register based on a selector string.
    ///
    /// # Type Parameters
    /// `T` - The type of the value to be retrieved.
    ///
    /// # Arguments
    /// * `reg_type` - The type of SIMD register to operate on.
    /// * `reg_index` - The index of the register.
    /// * `selector` - The string selector determining the range of bits to retrieve.
    ///
    /// # Returns
    /// The value of the specified type from the register, or `None` if invalid.
    pub fn get_by_selector<T: SectionCompatible>(&self, _reg_type: VecRegName, reg_index: usize, selector: &str) -> Option<T> {
        if let Some((a, b)) = extract_values(selector) {
            Some(self.simd_registers[reg_index].get_by_index(b, a))
        } else {
            None
        }
    }

    /// Sets a value in a specified SIMD register based on a selector string.
    ///
    /// # Type Parameters
    /// `T` - The type of the value to be set.
    ///
    /// # Arguments
    /// * `reg_type` - The type of SIMD register to operate on.
    /// * `reg_index` - The index of the register.
    /// * `selector` - The string selector determining the range of bits to set.
    /// * `value` - The value to set in the specified range.
    ///
    /// # Returns
    /// `true` if the operation was successful, `false` otherwise.
    pub fn set_by_selector<T: SectionCompatible>(&mut self, _reg_type: VecRegName, reg_index: usize, selector: &str, value: T) -> bool {
        if let Some((a, b)) = extract_values(selector) {
            self.simd_registers[reg_index].set_by_index(b, a, value);
            true
        } else {
            false
        }
    }

    /// Sets the value of a specified general-purpose register.
    ///
    /// Handles specific bits based on the register's type and size.
    ///
    /// # Arguments
    /// * `reg_name` - The name of the general-purpose register.
    /// * `value` - The value to set the register to.
    pub fn set_gpr_value(&mut self, reg_name: GPRName, value: u64) {
        register_set!(self; reg_name; value;
            RAX, EAX, AX, AL, AH,
            RBX, EBX, BX, BL, BH,
            RCX, ECX, CX, CL, CH,
            RDX, EDX, DX, DL, DH;
            R8, R8D, R8W, R8B,
            R9, R9D, R9W, R9B,
            R10, R10D, R10W, R10B,
            R11, R11D, R11W, R11B,
            R12, R12D, R12W, R12B,
            R13, R13D, R13W, R13B,
            R14, R14D, R14W, R14B,
            R15, R15D, R15W, R15B,
            RSP, ESP, SP, SPL,
            RBP, EBP, BP, BPL,
            RSI, ESI, SI, SIL,
            RDI, EDI, DI, DIL
        );
    }

    /// Retrieves the value of a specified general-purpose register.
    ///
    /// # Arguments
    /// * `reg_name` - The name of the general-purpose register.
    ///
    /// # Returns
    /// The current value of the specified register.
    pub fn get_gpr_value(&self, reg_name: GPRName) -> u64 {
        register_get!(self; reg_name;
            RAX, EAX, AX, AL, AH,
            RBX, EBX, BX, BL, BH,
            RCX, ECX, CX, CL, CH,
            RDX, EDX, DX, DL, DH;
            R8, R8D, R8W, R8B,
            R9, R9D, R9W, R9B,
            R10, R10D, R10W, R10B,
            R11, R11D, R11W, R11B,
            R12, R12D, R12W, R12B,
            R13, R13D, R13W, R13B,
            R14, R14D, R14W, R14B,
            R15, R15D, R15W, R15B,
            RSP, ESP, SP, SPL,
            RBP, EBP, BP, BPL,
            RSI, ESI, SI, SIL,
            RDI, EDI, DI, DIL
        )
    }

    /// Sets the value of a specified flags register.
    ///
    /// # Arguments
    /// * `reg_name` - The name of the flags register.
    /// * `value` - The value to set the flags register to.
    pub fn set_flags_value(&mut self, reg_name: FLAGSName, value: u64) {
        match reg_name {
            FLAGSName::RFLAGS => {
                self.rflags = value;
            },
            FLAGSName::EFLAGS => {
                self.rflags = (self.rflags & 0xFFFFFFFF_00000000) | (value & 0x00000000_FFFFFFFF);
            },
            FLAGSName::FLAGS => {
                self.rflags = (self.rflags & 0xFFFFFFFF_FFFF0000) | (value & 0x00000000_0000FFFF);
            }
        }
    }

    /// Retrieves the value of a specified flags register.
    ///
    /// # Arguments
    /// * `reg_name` - The name of the flags register.
    ///
    /// # Returns
    /// The current value of the
    pub fn get_flags_value(&self, reg_name: FLAGSName) -> u64 {
        match reg_name {
            FLAGSName::RFLAGS => {
                self.rflags
            },
            FLAGSName::EFLAGS => {
                self.rflags & 0x00000000_FFFFFFFF
            },
            FLAGSName::FLAGS => {
                self.rflags & 0x00000000_0000FFFF
            }
        }
    }

    /// Sets the value of a specified instruction pointer (IP) register.
    ///
    /// Handles specific bits based on the IP register's type and size. This method
    /// can be used to set the value of RIP, EIP, or IP registers.
    ///
    /// # Arguments
    /// * `reg_name` - The name of the IP register, which can be RIP, EIP, or IP.
    /// * `value` - The value to set the IP register to.
    pub fn set_ip_value(&mut self, reg_name: IPName, value: u64) {
        match reg_name {
            IPName::RIP => {
                self.rip = value;
            },
            IPName::EIP => {
                self.rip = value & 0x00000000_FFFFFFFF;
            },
            IPName::IP => {
                self.rip = value & 0x00000000_0000FFFF;
            }
        }
    }

    /// Retrieves the value of a specified instruction pointer (IP) register.
    ///
    /// Depending on the type of IP register queried (RIP, EIP, or IP), it returns
    /// the respective 64-bit, 32-bit, or 16-bit value.
    ///
    /// # Arguments
    /// * `reg_name` - The name of the IP register, which can be RIP, EIP, or IP.
    ///
    /// # Returns
    /// The current value of the specified IP register.
    pub fn get_ip_value(&self, reg_name: IPName) -> u64 {
        match reg_name {
            IPName::RIP => {
                self.rip
            },
            IPName::EIP => {
                self.rip & 0x00000000_FFFFFFFF
            },
            IPName::IP => {
                self.rip & 0x00000000_0000FFFF
            }
        }
    }
}
