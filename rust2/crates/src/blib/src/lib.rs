#![no_std]
#![feature(vec_into_raw_parts)]
#![feature(box_vec_non_null)]

extern crate alloc;

mod csr;
mod csr_defs;

pub mod ddi;
pub mod debug;
pub mod keyboard;
pub mod std_types;
pub mod term;

#[inline(always)]
pub fn halt() -> ! {
    csr::write::<{ csr_defs::CSR_HALT }>(0);
    loop {}
}

#[inline(always)]
pub fn set_heap_size(size: u32) {
    csr::write::<{ csr_defs::HEAP }>(size);
}
