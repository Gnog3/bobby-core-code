use core::fmt::{self, Write};

use crate::csr::{nop, write, write_ptr};
use crate::csr_defs::{DEBUG_CLEAR, DEBUG_LENGTH, DEBUG_NEWLINE, DEBUG_PRINT};

#[inline(always)]
pub fn print_u8(s: &[u8]) {
    write::<DEBUG_LENGTH>(s.len() as u32);
    write_ptr::<DEBUG_PRINT, _>(s.as_ptr());
    nop();
}

#[inline(always)]
pub fn print_str(s: &str) {
    print_u8(s.as_bytes());
}

#[inline(always)]
pub fn newline() {
    write::<DEBUG_NEWLINE>(0);
}

#[inline(always)]
pub fn print_str_newline(s: &str) {
    print_str(s);
    newline();
}

#[inline(always)]
pub fn clear() {
    write::<DEBUG_CLEAR>(0);
}

pub struct DebugWriter;

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if bytes[len - 1] == b'\n' {
            if len > 1 {
                print_u8(&bytes[0..len - 1]);
            }
            newline();
        } else {
            if !s.is_empty() {
                print_str(s);
            }
        }
        Ok(())
    }
}

pub fn print_args(args: fmt::Arguments) {
    let _ = DebugWriter.write_fmt(args);
}

#[macro_export]
macro_rules! print_err {
    ($($arg:tt)*) => ($crate::debug::print_args(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println_err {
    () => ($crate::debug::newline());
    ($($arg:tt)*) => {
        $crate::debug::print_args(format_args!($($arg)*));
        $crate::debug::newline();
    }
}
