use crate::{csr::read, csr_defs::CSR_KEYBOARD};

pub const KEY_0: u8 = 0x30;
pub const KEY_1: u8 = 0x31;
pub const KEY_2: u8 = 0x32;
pub const KEY_3: u8 = 0x33;
pub const KEY_4: u8 = 0x34;
pub const KEY_5: u8 = 0x35;
pub const KEY_6: u8 = 0x36;
pub const KEY_7: u8 = 0x37;
pub const KEY_8: u8 = 0x38;
pub const KEY_9: u8 = 0x39;

pub const KEY_ENTER: u8 = 0x10;

pub const KEY_UP: u8 = 0x11;
pub const KEY_LEFT: u8 = 0x12;
pub const KEY_DOWN: u8 = 0x13;
pub const KEY_RIGHT: u8 = 0x14;

pub const KEY_BACKSPACE: u8 = 0x0D;

pub const KEY_A: u8 = 0x40;
pub const KEY_B: u8 = 0x41;
pub const KEY_C: u8 = 0x42;
pub const KEY_D: u8 = 0x43;
pub const KEY_E: u8 = 0x44;
pub const KEY_F: u8 = 0x45;
pub const KEY_G: u8 = 0x46;
pub const KEY_H: u8 = 0x47;
pub const KEY_I: u8 = 0x48;
pub const KEY_J: u8 = 0x49;
pub const KEY_K: u8 = 0x4A;
pub const KEY_L: u8 = 0x4B;
pub const KEY_M: u8 = 0x4C;
pub const KEY_N: u8 = 0x4D;
pub const KEY_O: u8 = 0x4E;
pub const KEY_P: u8 = 0x4F;
pub const KEY_Q: u8 = 0x50;
pub const KEY_R: u8 = 0x51;
pub const KEY_S: u8 = 0x52;
pub const KEY_T: u8 = 0x53;
pub const KEY_U: u8 = 0x54;
pub const KEY_V: u8 = 0x55;
pub const KEY_W: u8 = 0x56;
pub const KEY_X: u8 = 0x57;
pub const KEY_Y: u8 = 0x58;
pub const KEY_Z: u8 = 0x59;

pub const KEY_DOT: u8 = 0x70;

#[inline(always)]
pub fn read_key() -> u8 {
    read::<CSR_KEYBOARD>() as u8
}

pub fn handle_control(_key: u8) -> bool {
    false
}

pub fn get_char(key: u8) -> u8 {
    if key >= KEY_0 && key <= KEY_9 {
        key
    } else if key == KEY_DOT {
        b'.'
    } else {
        log::error!("Unknown key {:X}", key);
        0
    }
}