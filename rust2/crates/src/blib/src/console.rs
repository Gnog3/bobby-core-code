use alloc::string::String;

use crate::std_types::{RStr, RString};

unsafe extern "C" {
    safe fn console_clear();
    safe fn console_print(s: RStr);
    safe fn console_newline();
    safe fn console_input() -> RString;
}

pub fn clear() {
    console_clear();
}

pub fn print(s: &str) {
    console_print(RStr::from_str(s));
}

pub fn newline() {
    console_newline();
}

pub fn input() -> String {
    console_input().to_string()
}
