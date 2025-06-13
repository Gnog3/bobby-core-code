#![no_std]

use blib::std_types::RStr;

unsafe extern "C" {
    safe fn init_current_dir() -> RStr<'static>;
    safe fn init_arg() -> RStr<'static>;
}

pub fn current_dir() -> &'static str {
    init_current_dir().as_str()
}

pub fn arg() -> &'static str {
    init_arg().as_str()
}
