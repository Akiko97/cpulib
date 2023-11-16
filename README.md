# cpulib

A CPU emulator library designed to simulate the CPU context, with a specific focus on SIMD operations.

# Usage

```rust
use cpulib::{ CPU, Utilities, u256, u512, VecRegName, GPRName, FLAGSName, IPName };

let mut cpu = CPU::default();

cpu.registers.set_bit(VecRegName::XMM, 0, 127, true);
println!("{:?}", cpu.registers.get_bit(VecRegName::XMM, 0, 127));
```
