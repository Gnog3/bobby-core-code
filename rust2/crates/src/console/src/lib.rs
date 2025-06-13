#![no_std]

extern crate alloc;
use alloc::string::String;

use blib::std_types::{RStr, RString};

unsafe extern "C" {
    safe fn console_clear();
    safe fn console_print(s: RStr);
    safe fn console_newline();
    safe fn console_input() -> RString;
}

#[inline(always)]
pub fn clear() {
    console_clear();
}

#[inline(always)]
pub fn print(s: &str) {
    console_print(RStr::from_str(s));
}

#[inline(always)]
pub fn newline() {
    console_newline();
}

#[inline(always)]
pub fn input() -> String {
    console_input().to_string()
}

#[macro_export]
macro_rules! print {
    ($($item:expr),+) => {
        $(
            $crate::print($item);
        )+
    };
}

#[macro_export]
macro_rules! println {
    ($($item:expr),+) => {
        $(
            $crate::print($item);
        )+
        $crate::newline();
    }
}
