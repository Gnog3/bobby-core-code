#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

mod counter;

global_asm!(include_str!("init.s"));

/// A panic handler is required in Rust, this is probably the most basic one possible
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Main program function
#[no_mangle]
extern "C" fn main() -> () {
    // Example: Create a counter peripheral with base address 0x8000_0000
    // let mut counter: Counter<0x8000_0000> = Counter::new();
    // counter.set_control_reg(0xaaaa);
    // let c_s = counter.get_status_reg();
    // let c_c = counter.get_control_reg();
    // let c_v = counter.get_value();
    let ptr = 0x8200_0000 as *mut u32;
    unsafe {
        let a = ptr.read_volatile();
        // let b = ptr.read_volatile();
        let c = a * 5;

        ptr.write_volatile(c);
    }
}
