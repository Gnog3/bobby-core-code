use core::alloc::{GlobalAlloc, Layout};

use crate::HEAP;

pub unsafe extern "C" fn heap_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, align);
        HEAP.alloc(layout)
    }
}

pub unsafe extern "C" fn heap_dealloc(ptr: *mut u8, size: usize, align: usize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, align);
        HEAP.dealloc(ptr, layout);
    }
}
