use blib::std_types::{RStr, RString};

pub extern "C" fn console_clear() {
    super::clear();
}

pub extern "C" fn console_print(s: RStr) {
    super::print(s.as_str());
}

pub extern "C" fn console_newline() {
    super::newline();
}

pub extern "C" fn console_input() -> RString {
    RString::from_string(super::input())
}
