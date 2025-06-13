#![no_std]

unsafe extern "C" {
    safe fn keyboard_handle_control(key: u8) -> bool;
    safe fn keyboard_get_char(key: u8) -> u8;
}

#[inline(always)]
pub fn handle_control(key: u8) -> bool {
    keyboard_handle_control(key)
}

#[inline(always)]
pub fn get_char(key: u8) -> u8 {
    keyboard_get_char(key)
}
