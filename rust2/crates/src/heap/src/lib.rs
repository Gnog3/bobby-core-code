#![no_std]

use core::alloc::GlobalAlloc;

unsafe extern "C" {
    safe fn heap_alloc(size: usize, align: usize) -> *mut u8;
    safe fn heap_dealloc(ptr: *mut u8, size: usize, align: usize);
}
struct Heap;

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        heap_alloc(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        heap_dealloc(ptr, layout.size(), layout.align());
    }
}

#[global_allocator]
static HEAP: Heap = Heap;
