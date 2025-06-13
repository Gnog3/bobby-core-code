#![no_std]

mod critical_section;
mod csr;
mod csr_defs;
pub mod ddi;
pub mod debug;
pub mod fs;
mod heap;
mod log_impl;
use core::{arch::global_asm, panic::PanicInfo};
pub mod keyboard;
pub mod term;

use csr_defs::CSR_HALT;

global_asm!(include_str!("init.s"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    csr::write::<CSR_HALT>(0);
    loop {}
}

extern "C" {
    fn main2();
}

#[no_mangle]
extern "C" fn startup() -> ! {
    // unsafe {
    //     log::set_logger_racy(&log_impl::LOGGER).unwrap();
    //     log::set_max_level_racy(log::LevelFilter::Trace);
    // };
    debug::clear();
    log::info!("Startuem");
    // unsafe {
    //     heap::HEAP.init();
    // }
    unsafe {
        main2();
    }
    log::info!("Halt");
    csr::write::<CSR_HALT>(0);
    loop {}
}
