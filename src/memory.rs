extern crate primitive_types;
use primitive_types::U256 as u256;
use primitive_types::U512 as u512;

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// Trait for memory I/O operations, allowing types to be read from and written
/// to byte arrays, along with querying their memory size.
pub trait MemoryIO {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn size() -> usize;
}

/// Macro to implement `MemoryIO` trait for basic unsigned integer types.
/// It provides methods to convert between the type and byte arrays using little endian format.
macro_rules! impl_memory_io {
    ($t:ty, $type_str:ident, $size:expr) => {
        impl MemoryIO for $t {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut rdr = std::io::Cursor::new(bytes);
                match stringify!($type_str) {
                    "u8" => rdr.read_u8().unwrap() as $t,
                    "u16" => rdr.read_u16::<LittleEndian>().unwrap() as $t,
                    "u32" => rdr.read_u32::<LittleEndian>().unwrap() as $t,
                    "u64" => rdr.read_u64::<LittleEndian>().unwrap() as $t,
                    "u128" => rdr.read_u128::<LittleEndian>().unwrap() as $t,
                    _ => panic!("Unsupported type"),
                }
            }

            fn to_bytes(&self) -> Vec<u8> {
                let mut wtr = vec![];
                match stringify!($type_str) {
                    "u8" => wtr.write_u8(*self as u8).unwrap(),
                    "u16" => wtr.write_u16::<LittleEndian>(*self as u16).unwrap(),
                    "u32" => wtr.write_u32::<LittleEndian>(*self as u32).unwrap(),
                    "u64" => wtr.write_u64::<LittleEndian>(*self as u64).unwrap(),
                    "u128" => wtr.write_u128::<LittleEndian>(*self as u128).unwrap(),
                    _ => panic!("Unsupported type"),
                };
                wtr
            }

            fn size() -> usize {
                $size
            }
        }
    };
}

impl_memory_io!(u8, u8, 1);
impl_memory_io!(u16, u16, 2);
impl_memory_io!(u32, u32, 4);
impl_memory_io!(u64, u64, 8);
impl_memory_io!(u128, u128, 16);

/// Implements `MemoryIO` for `u256` type, enabling conversion between `u256` and byte arrays.
/// The conversion is handled in little endian format.
impl MemoryIO for u256 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u256::from_little_endian(bytes)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut wtr = vec![0; 32];
        self.to_little_endian(&mut wtr);
        wtr
    }

    fn size() -> usize {
        32
    }
}

/// Implements `MemoryIO` for `u512` type, enabling conversion between `u512` and byte arrays.
/// The conversion is handled in little endian format.
impl MemoryIO for u512 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u512::from_little_endian(bytes)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut wtr = vec![0; 64];
        self.to_little_endian(&mut wtr);
        wtr
    }

    fn size() -> usize {
        64
    }
}

const DEFAULT_SIZE: usize = 512; // 512 bytes

/// Represents a segment of memory with a start address and data content.
/// Used to manage discrete blocks of memory within a larger memory structure.
struct MemorySegment {
    start_address: usize,
    data: Vec<u8>,
}

/// Represents a memory model with segmented memory blocks.
/// Provides functionality for reading and writing data to specific memory addresses.
pub struct Memory {
    segments: Vec<MemorySegment>,
    pub base_address: usize,
}

impl Memory {
    /// Creates a new instance of `Memory`.
    ///
    /// Initializes an empty vector of `MemorySegment` and sets the base address for memory calculations.
    ///
    /// # Arguments
    /// * `base` - The base address from which all memory addresses will be calculated.
    ///
    /// # Returns
    /// A new `Memory` instance with the specified base address.
    pub fn new(base: usize) -> Self {
        Memory {
            segments: Vec::new(),
            base_address: base,
        }
    }

    /// Searches for a memory segment that contains a specified real address.
    ///
    /// Iterates through the memory segments to find a segment where the real address falls within
    /// the segment's range (from its start address to the end of its data).
    ///
    /// # Arguments
    /// * `real_address` - The real memory address to locate within the segments.
    ///
    /// # Returns
    /// An `Option<usize>` representing the index of the found memory segment in the `segments` vector.
    /// Returns `None` if no suitable segment is found.
    fn find_segment(&self, real_address: usize) -> Option<usize> {
        for (index, segment) in self.segments.iter().enumerate() {
            if real_address >= segment.start_address && real_address < segment.start_address + segment.data.len() {
                return Some(index);
            }
        }
        None
    }

    /// Reads a single byte from memory at a given address.
    ///
    /// Calculates the real address by subtracting the base address from the given address.
    /// If the address is within a memory segment, returns the byte at the calculated offset within the segment.
    /// If the address is not mapped to any segment, returns 0.
    ///
    /// # Arguments
    /// * `address` - The address from which to read the byte.
    ///
    /// # Returns
    /// The byte value at the given address, or 0 if the address is not mapped.
    fn read_byte(&self, address: usize) -> u8 {
        let real_address = address - self.base_address;
        if let Some(index) = self.find_segment(real_address) {
            self.segments[index].data[real_address - self.segments[index].start_address]
        } else {
            // return 0 if the address is not found
            0
        }
    }

    /// Writes a single byte to memory at a given address.
    ///
    /// Calculates the real address by subtracting the base address from the given address.
    /// If a segment containing the address exists, updates the byte at the specific offset.
    /// If no segment contains the address, a new segment is created and added to the memory.
    /// Segments are automatically merged if they become contiguous after the write operation.
    ///
    /// # Arguments
    /// * `address` - The address at which to write the byte.
    /// * `value` - The byte value to write.
    fn write_byte(&mut self, address: usize, value: u8) {
        let real_address = address - self.base_address;
        if let Some(index) = self.find_segment(real_address) {
            let start = self.segments[index].start_address;
            self.segments[index].data[real_address - start] = value;
        } else {
            let adjusted_address = (real_address / DEFAULT_SIZE) * DEFAULT_SIZE;
            let mut new_data = Vec::with_capacity(DEFAULT_SIZE);
            new_data.resize(DEFAULT_SIZE, 0);
            new_data[real_address - adjusted_address] = value;
            let new_segment = MemorySegment {
                start_address: adjusted_address,
                data: new_data,
            };
            self.segments.push(new_segment);
            // sort by address
            self.segments.sort_by(|a, b| a.start_address.cmp(&b.start_address));
        }
        // merge segments if they are contiguous
        let mut i = 0;
        while i + 1 < self.segments.len() {
            if self.segments[i].start_address + self.segments[i].data.len() == self.segments[i + 1].start_address {
                let next = self.segments.remove(i + 1);
                self.segments[i].data.extend(next.data);
            } else {
                i += 1;
            }
        }
    }

    /// Reads a value of type `T` from memory starting at a given address.
    ///
    /// Reads bytes sequentially starting from the address and constructs a value of type `T` using `MemoryIO` trait.
    /// The number of bytes read is determined by the size of type `T`.
    ///
    /// # Type Parameters
    /// * `T` - The type implementing `MemoryIO` that determines how bytes are read and interpreted.
    ///
    /// # Arguments
    /// * `address` - The starting address from which to read the bytes.
    ///
    /// # Returns
    /// A value of type `T` constructed from the read bytes.
    pub fn read<T: MemoryIO>(&self, address: usize) -> T {
        let mut bytes = Vec::new();
        for i in 0..T::size() {
            bytes.push(self.read_byte(address + i));
        }
        T::from_bytes(&bytes)
    }

    /// Writes a value of type `T` to memory starting at a given address.
    ///
    /// Converts the value to a byte array using `MemoryIO` trait and writes the bytes sequentially to memory.
    /// Each byte is written to consecutive memory addresses starting from the specified address.
    ///
    /// # Type Parameters
    /// * `T` - The type implementing `MemoryIO` that determines how the value is converted to bytes.
    ///
    /// # Arguments
    /// * `address` - The starting address at which to write the bytes.
    /// * `value` - The value of type `T` to write to memory.
    pub fn write<T: MemoryIO>(&mut self, address: usize, value: T) {
        let bytes = value.to_bytes();
        for (i, byte) in bytes.iter().enumerate() {
            self.write_byte(address + i, *byte);
        }
    }

    /// Reads a vector of values of type `T` from memory starting at a given address.
    ///
    /// Reads multiple values sequentially from memory. The number of values read is determined by `number_of_value`.
    /// Each value is read from a block of memory with a size equal to `T::size()`.
    ///
    /// # Type Parameters
    /// * `T` - The type implementing `MemoryIO` that determines how bytes are read and interpreted.
    ///
    /// # Arguments
    /// * `address` - The starting address from which to begin reading values.
    /// * `number_of_value` - The number of values to read.
    ///
    /// # Returns
    /// A vector of values of type `T`.
    pub fn read_vec<T: MemoryIO>(&self, address: usize, number_of_value: usize) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        for i in 0..number_of_value {
            result.push(self.read(address + i * T::size()));
        }
        result
    }

    /// Writes a vector of values of type `T` to memory starting at a given address.
    ///
    /// Sequentially writes each value in the vector to memory. The address for each subsequent value
    /// is offset by `T::size()` bytes from the previous one.
    ///
    /// # Type Parameters
    /// * `T` - The type implementing `MemoryIO` and `Clone` that determines how values are converted to bytes.
    ///
    /// # Arguments
    /// * `address` - The starting address at which to begin writing values.
    /// * `values` - The vector of values to write to memory.
    pub fn write_vec<T: MemoryIO + Clone>(&mut self, address: usize, values: Vec<T>) {
        for (i, value) in values.iter().enumerate() {
            self.write(address + i * T::size(), value.clone());
        }
    }
}
