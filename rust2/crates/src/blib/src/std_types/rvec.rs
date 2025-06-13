use core::{mem::ManuallyDrop, ptr::NonNull};

use alloc::vec::Vec;

#[repr(C)]
pub struct RVec<T> {
    buffer: NonNull<T>,
    capacity: usize,
    length: usize,
}

impl<T> RVec<T> {
    #[inline(always)]
    pub fn from_vec(vec: Vec<T>) -> Self {
        unsafe {
            let (buffer, length, capacity) = vec.into_raw_parts();
            Self {
                buffer: NonNull::new_unchecked(buffer),
                capacity,
                length,
            }
        }
    }

    #[inline(always)]
    pub fn to_vec(self) -> Vec<T> {
        let vec = ManuallyDrop::new(self);
        unsafe { Vec::from_parts(vec.buffer, vec.length, vec.capacity) }
    }
}

impl<T> Drop for RVec<T> {
    fn drop(&mut self) {
        panic!("RVec must not be dropped");
    }
}
