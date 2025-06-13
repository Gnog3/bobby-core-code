use core::mem::MaybeUninit;

use bytemuck::{bytes_of, NoUninit, Pod};

use crate::Offset;

#[allow(unused)]
struct Words<T>(T);

#[allow(unused)]
impl<T> Words<T> {
    const ASSERT: () = assert!(size_of::<T>() % 4 == 0 && align_of::<T>() % 4 == 0);
}

pub trait Storage: Clone {
    fn read(&self, offset: Offset, buf: &mut [MaybeUninit<u8>]);

    // optimized read when buf.len() % 4 == 0
    fn read_words(&self, offset: Offset, buf: &mut [MaybeUninit<u8>]) {
        self.read(offset, buf);
    }

    fn write(&self, offset: Offset, buf: &[u8]);

    // optimized write when buf.len % 4 == 0
    fn write_words(&self, offset: Offset, buf: &[u8]) {
        self.write(offset, buf);
    }

    fn capacity(&self) -> u32;
}

pub(crate) trait StorageExt: Storage {
    fn read_t<T: Pod>(&self, offset: Offset) -> T {
        let _ = Words::<T>::ASSERT;
        let mut uninit: MaybeUninit<T> = MaybeUninit::uninit();
        self.read_words(offset, uninit.as_bytes_mut());
        unsafe { uninit.assume_init() }
    }

    fn write_t<T: NoUninit>(&self, offset: Offset, t: &T) {
        let _ = Words::<T>::ASSERT;
        self.write_words(offset, bytes_of(t));
    }
}

impl<T: Storage> StorageExt for T {}

#[cfg(test)]
pub mod tests {
    extern crate alloc;
    use alloc::{rc::Rc, vec, vec::Vec};
    use core::{cell::RefCell, mem::MaybeUninit};

    use super::Storage;

    #[derive(Clone)]
    pub struct HeapStorage {
        vec: Rc<RefCell<Vec<u8>>>,
    }

    impl HeapStorage {
        pub fn new() -> Self {
            Self {
                vec: Rc::new(RefCell::new(vec![111; 16 * 1024 * 1024])),
            }
        }
    }

    impl Storage for HeapStorage {
        fn read(&self, offset: u32, buf: &mut [core::mem::MaybeUninit<u8>]) {
            let vec = self.vec.borrow();
            MaybeUninit::copy_from_slice(buf, &vec[offset as usize..offset as usize + buf.len()]);
        }

        fn write(&self, offset: u32, buf: &[u8]) {
            let mut vec = self.vec.borrow_mut();
            vec[offset as usize..offset as usize + buf.len()].copy_from_slice(buf);
        }

        fn capacity(&self) -> u32 {
            self.vec.borrow().len() as u32
        }
    }
}
