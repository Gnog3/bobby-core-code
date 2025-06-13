/// The different sizes used for memory accesses
#[derive(Clone, Copy)]
#[repr(usize)]
pub enum MemAccessSize {
    /// 8 bits
    Byte = 1,
    /// 16 bits
    HalfWord = 2,
    /// 32 bits
    Word = 4,
}

/// A trait for objects which implement memory operations
// pub trait Memory {
//     /// Read `size` bytes from `addr`.
//     ///
//     /// `addr` must be aligned to `size`.
//     /// Returns `None` if `addr` doesn't exist in this memory.
//     fn read_mem(&mut self, addr: u32, size: MemAccessSize) -> Option<u32>;

//     /// Write `size` bytes of `store_data` to `addr`
//     ///
//     /// `addr` must be aligned to `size`.
//     /// Returns `true` if write succeeds.
//     fn write_mem(&mut self, addr: u32, size: MemAccessSize, store_data: u32) -> bool;
// }

pub struct Memory {
    pub vec: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        assert_eq!(size % 4, 0);

        Self { vec: vec![0; size] }
    }

    pub fn read(&self, addr: u32, osize: MemAccessSize) -> u32 {
        let addr = addr as usize;
        let size = osize as usize;

        assert_eq!(addr % size, 0, "Invalid alignment for read mem operation");
        assert!(addr + size <= self.vec.len(), "read is out of range");

        match osize {
            MemAccessSize::Byte => self.vec[addr].into(),
            MemAccessSize::HalfWord => {
                u16::from_le_bytes(self.vec[addr..addr + size].try_into().unwrap()).into()
            }
            MemAccessSize::Word => {
                u32::from_le_bytes(self.vec[addr..addr + size].try_into().unwrap())
            }
        }
    }

    pub fn write(&mut self, addr: u32, osize: MemAccessSize, data: u32) {
        let addr = addr as usize;
        let size = osize as usize;

        assert_eq!(addr % size, 0, "Invalid alignment for write mem operation");
        assert!(addr + size <= self.vec.len(), "write is out of range");
        let data = data.to_le_bytes();
        self.vec[addr..addr + size].copy_from_slice(&data[..size]);
    }

    pub fn flash(&mut self, data: &[u8]) {
        self.vec[..data.len()].copy_from_slice(data);
    }
}
