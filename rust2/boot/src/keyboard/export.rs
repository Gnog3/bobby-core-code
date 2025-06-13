use super::{get_char, handle_control};

pub extern "C" fn keyboard_handle_control(key: u8) -> bool {
    handle_control(key)
}

pub extern "C" fn keyboard_get_char(key: u8) -> u8 {
    get_char(key)
}
