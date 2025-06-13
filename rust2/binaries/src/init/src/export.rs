use core::mem::transmute;

use blib::std_types::RStr;

static mut CURRENT_DIR: &str = "";
static mut ARG: &str = "";

pub fn set_current_dir(current_dir: &str) {
    unsafe {
        let dir_g = &raw mut CURRENT_DIR;
        dir_g.write_volatile(transmute(current_dir));
    }
}

pub fn set_arg(arg: &str) {
    unsafe {
        let arg_g = &raw mut ARG;
        arg_g.write_volatile(transmute(arg));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn init_current_dir() -> RStr<'static> {
    let str = unsafe {
        let dir_g = &raw const CURRENT_DIR;
        dir_g.read_volatile()
    };
    RStr::from_str(str)
}

#[unsafe(no_mangle)]
pub extern "C" fn init_arg() -> RStr<'static> {
    let str = unsafe {
        let arg_g = &raw const ARG;
        arg_g.read_volatile()
    };
    RStr::from_str(str)
}
