#![no_std]

mod critical_section;
pub mod export;

use core::alloc::GlobalAlloc;

use blib::debug::{print_str, print_str_newline};
use embedded_alloc::LlffHeap;

#[global_allocator]
pub static HEAP: Heap = Heap::empty();

pub struct Heap {
    heap: LlffHeap,
}

impl Heap {
    pub const fn empty() -> Self {
        Heap {
            heap: LlffHeap::empty(),
        }
    }

    pub unsafe fn init(&self) {
        const START_ADDRESS: usize = 2 * 1024 * 1024;
        const HEAP_SIZE: usize = 1024 * 1024; // a megabyte
        print_str("Initializing heap... ");
        unsafe { self.heap.init(START_ADDRESS, HEAP_SIZE) }
        print_str_newline("OK.");
    }

    pub fn used(&self) -> usize {
        self.heap.used()
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = unsafe { self.heap.alloc(layout) };
        if ptr.is_null() {
            print_str_newline("Failed to allocate.");
        }
        blib::set_heap_size(self.used() as _);
        // blib::println_err!(
        //     "Allocated {} bytes. Used {} bytes.",
        //     layout.size(),
        //     self.heap.used()
        // );
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        unsafe {
            self.heap.dealloc(ptr, layout);
        }
        blib::set_heap_size(self.used() as _);
        // blib::println_err!(
        //     "Freed {} bytes. Used {} bytes.",
        //     layout.size(),
        //     self.heap.used()
        // );
    }
}
