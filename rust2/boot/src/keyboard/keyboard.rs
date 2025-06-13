use blib::keyboard::*;

pub const CHAR_MAP: [u8; 128] = {
    let mut map = [0u8; 128];

    map[KEY_SPACE as usize] = b' ';

    map[KEY_0 as usize] = b'0';
    map[KEY_1 as usize] = b'1';
    map[KEY_2 as usize] = b'2';
    map[KEY_3 as usize] = b'3';
    map[KEY_4 as usize] = b'4';
    map[KEY_5 as usize] = b'5';
    map[KEY_6 as usize] = b'6';
    map[KEY_7 as usize] = b'7';
    map[KEY_8 as usize] = b'8';
    map[KEY_9 as usize] = b'9';

    map[KEY_A as usize] = b'a';
    map[KEY_B as usize] = b'b';
    map[KEY_C as usize] = b'c';
    map[KEY_D as usize] = b'd';
    map[KEY_E as usize] = b'e';
    map[KEY_F as usize] = b'f';
    map[KEY_G as usize] = b'g';
    map[KEY_H as usize] = b'h';
    map[KEY_I as usize] = b'i';
    map[KEY_J as usize] = b'j';
    map[KEY_K as usize] = b'k';
    map[KEY_L as usize] = b'l';
    map[KEY_M as usize] = b'm';
    map[KEY_N as usize] = b'n';
    map[KEY_O as usize] = b'o';
    map[KEY_P as usize] = b'p';
    map[KEY_Q as usize] = b'q';
    map[KEY_R as usize] = b'r';
    map[KEY_S as usize] = b's';
    map[KEY_T as usize] = b't';
    map[KEY_U as usize] = b'u';
    map[KEY_V as usize] = b'v';
    map[KEY_W as usize] = b'w';
    map[KEY_X as usize] = b'x';
    map[KEY_Y as usize] = b'y';
    map[KEY_Z as usize] = b'z';

    map[KEY_MINUS as usize] = b'-';
    map[KEY_EQUALS as usize] = b'=';
    map[KEY_LBRACKET as usize] = b'[';
    map[KEY_RBRACKET as usize] = b']';
    map[KEY_SEMICOLON as usize] = b';';
    map[KEY_APOSTROPHE as usize] = b'\'';
    map[KEY_COMMA as usize] = b',';
    map[KEY_PERIOD as usize] = b'.';
    map[KEY_SLASH as usize] = b'/';
    map[KEY_TILDE as usize] = b'`';
    map[KEY_BACKSLASH as usize] = b'\\';
    map
};

pub const SHIFT_CHAR_MAP: [u8; 128] = {
    let mut map = [0u8; 128];

    map[KEY_SPACE as usize] = b' ';
    
    map[KEY_0 as usize] = b')';
    map[KEY_1 as usize] = b'!';
    map[KEY_2 as usize] = b'@';
    map[KEY_3 as usize] = b'#';
    map[KEY_4 as usize] = b'$';
    map[KEY_5 as usize] = b'%';
    map[KEY_6 as usize] = b'^';
    map[KEY_7 as usize] = b'&';
    map[KEY_8 as usize] = b'*';
    map[KEY_9 as usize] = b'(';

    map[KEY_A as usize] = b'A';
    map[KEY_B as usize] = b'B';
    map[KEY_C as usize] = b'C';
    map[KEY_D as usize] = b'D';
    map[KEY_E as usize] = b'E';
    map[KEY_F as usize] = b'F';
    map[KEY_G as usize] = b'G';
    map[KEY_H as usize] = b'H';
    map[KEY_I as usize] = b'I';
    map[KEY_J as usize] = b'J';
    map[KEY_K as usize] = b'K';
    map[KEY_L as usize] = b'L';
    map[KEY_M as usize] = b'M';
    map[KEY_N as usize] = b'N';
    map[KEY_O as usize] = b'O';
    map[KEY_P as usize] = b'P';
    map[KEY_Q as usize] = b'Q';
    map[KEY_R as usize] = b'R';
    map[KEY_S as usize] = b'S';
    map[KEY_T as usize] = b'T';
    map[KEY_U as usize] = b'U';
    map[KEY_V as usize] = b'V';
    map[KEY_W as usize] = b'W';
    map[KEY_X as usize] = b'X';
    map[KEY_Y as usize] = b'Y';
    map[KEY_Z as usize] = b'Z';

    map[KEY_MINUS as usize] = b'_';
    map[KEY_EQUALS as usize] = b'+';
    map[KEY_LBRACKET as usize] = b'{';
    map[KEY_RBRACKET as usize] = b'}';
    map[KEY_SEMICOLON as usize] = b':';
    map[KEY_APOSTROPHE as usize] = b'"';
    map[KEY_COMMA as usize] = b'<';
    map[KEY_PERIOD as usize] = b'>';
    map[KEY_SLASH as usize] = b'?';
    map[KEY_TILDE as usize] = b'~';
    map[KEY_BACKSLASH as usize] = b'|';
    map
};

static mut SHIFT_PRESSED: bool = false;

pub fn reset_shift() {
    unsafe {
        *&raw mut SHIFT_PRESSED = false;
    }
}

pub fn handle_control(key: u8) -> bool {
    if (key & 0x7F) == KEY_SHIFT {
        unsafe {
            *&raw mut SHIFT_PRESSED = (key & 0x80) != 0;
        }
        true
    } else {
        false
    }
}

pub fn get_char(key: u8) -> u8 {
    let shift_pressed = unsafe { *&raw mut SHIFT_PRESSED };
    let chars = if shift_pressed {
        &SHIFT_CHAR_MAP
    } else {
        &CHAR_MAP
    };
    let key = key & 0x7F;
    chars[key as usize]
}