use core::alloc::GlobalAlloc;

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
        log::trace!("Initializing heap...");
        unsafe { self.heap.init(START_ADDRESS, HEAP_SIZE) }
        log::trace!("Heap initialized.");
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        log::trace!("Allocating {} bytes...", layout.size());
        let ptr = unsafe { self.heap.alloc(layout) };
        if ptr.is_null() {
            log::error!("Failed to allocate {} bytes", layout.size());
        } else {
            log::trace!(
                "Allocated at {:p}. Total alloc: {} bytes",
                ptr,
                self.heap.used()
            );
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        log::trace!("Freeing {:p} ({} bytes)...", ptr, layout.size());
        unsafe { self.heap.dealloc(ptr, layout) }
        log::trace!("Freed. Total alloc: {} bytes", self.heap.used())
    }
}
