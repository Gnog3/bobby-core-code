use core::{fmt::Write, panic::PanicInfo};

use blib::{halt, println_err};

use crate::panic_screen::{end_panic, start_panic, PanicScreenWriter};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // ThisBinaryPanics();
    println_err!("Panic in boot: {}", _info);
    start_panic();
    let mut writer = PanicScreenWriter::new();
    writer.write_u8("Panic in boot".as_bytes());
    writer.newline();

    let _ = write!(writer, "{}", _info);
    end_panic();
    halt();
}
