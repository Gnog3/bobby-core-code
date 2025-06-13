#![no_std]

use core::panic::PanicInfo;

use blib::{halt, println_err};

unsafe extern "C" {
    safe fn ThisBinaryPanics();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println_err!("Panic in binary");
    halt();
}
