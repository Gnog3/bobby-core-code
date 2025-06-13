use core::str;

use alloc::string::String;
use blib::{
    ddi::{self, Pos},
    keyboard::{KEY_BACKSPACE, KEY_ENTER, read_key},
    term,
};

use crate::keyboard;

pub const SCREEN_WIDTH: u16 = 640;
pub const SCREEN_HEIGHT: u16 = 480;
pub const CHAR_WIDTH: u16 = 6;
pub const CHAR_HEIGHT: u16 = 8;

pub const MAX_CHARS_ON_LINE: u16 = SCREEN_WIDTH / CHAR_WIDTH;
pub const MAX_LINES: u16 = SCREEN_HEIGHT / CHAR_HEIGHT;

pub const MAX_X: u16 = MAX_CHARS_ON_LINE * CHAR_WIDTH;
pub const MAX_Y: u16 = MAX_LINES * CHAR_HEIGHT;

pub const LAST_X: u16 = MAX_X - CHAR_WIDTH;

pub const TEXT_COLOR: u32 = 0xFFFFFF;
pub const BACKGROUND_COLOR: u32 = 0;
pub const CURSOR_COLOR: u32 = 0xFF0000;

fn scroll(height: u16) {
    ddi::copy(
        Pos::ZERO,
        Pos::new(0, height),
        Pos::new(SCREEN_WIDTH, SCREEN_HEIGHT - height),
    );
    ddi::rectangle(
        Pos::new(0, SCREEN_HEIGHT - height),
        Pos::new(SCREEN_WIDTH, height),
        BACKGROUND_COLOR,
    );
}

pub struct Console {
    next_char_pos: Pos,
    chars_on_current_line: u16,

    cursor_pos: u32,
}

impl Console {
    pub const fn new() -> Self {
        Self {
            next_char_pos: Pos::ZERO,
            chars_on_current_line: 0,

            cursor_pos: 0,
        }
    }

    pub fn clear(&mut self) {
        ddi::floodfill(BACKGROUND_COLOR);
        self.next_char_pos = Pos::ZERO;
        self.chars_on_current_line = 0;
        self.cursor_pos = 0;
    }

    fn draw_cursor(&self, visible: bool) {
        let color = if visible {
            CURSOR_COLOR
        } else {
            BACKGROUND_COLOR
        };
        ddi::rectangle(
            Pos::new(self.next_char_pos.x, self.next_char_pos.y + 7),
            Pos::new(5, 1),
            color,
        );
    }

    fn handle_overflow(&mut self) {
        if self.next_char_pos.x >= MAX_X {
            self.next_char_pos.x = 0;
            self.next_char_pos.y += CHAR_HEIGHT;
            self.chars_on_current_line = 0;
        }
        if self.next_char_pos.y >= MAX_Y {
            scroll(CHAR_HEIGHT);
            self.next_char_pos.y -= CHAR_HEIGHT;
        }
    }

    pub fn print(&mut self, s: &str) {
        let mut slice = s.as_bytes();
        term::set_color(TEXT_COLOR);
        while !slice.is_empty() {
            let chars_to_print =
                (MAX_CHARS_ON_LINE - self.chars_on_current_line).min(slice.len() as _);
            term::print_slice(self.next_char_pos, &slice[0..chars_to_print as usize]);
            slice = &slice[chars_to_print as _..];
            self.chars_on_current_line += chars_to_print;
            self.next_char_pos.x += chars_to_print * CHAR_WIDTH;
            self.handle_overflow();
        }
    }

    pub fn newline(&mut self) {
        self.next_char_pos.x = 0;
        self.next_char_pos.y += CHAR_HEIGHT;
        self.chars_on_current_line = 0;
        self.handle_overflow();
    }

    pub fn input(&mut self) -> String {
        let mut s = String::with_capacity(16);
        let mut len = 0;
        self.draw_cursor(true);
        loop {
            let key = read_key();

            if keyboard::handle_control(key) {
                continue;
            }
            if (key >> 7) == 0 {
                continue;
            }
            let key = key & 0x7f;
            if key == KEY_ENTER {
                break;
            }
            if key == KEY_BACKSPACE && len != 0 {
                self.draw_cursor(false);
                if self.next_char_pos.x == 0 {
                    self.next_char_pos.x = LAST_X;
                    self.next_char_pos.y -= CHAR_HEIGHT;
                    self.chars_on_current_line = MAX_CHARS_ON_LINE - 1;
                } else {
                    self.next_char_pos.x -= CHAR_WIDTH;
                    self.chars_on_current_line -= 1;
                }
                len -= 1;
                unsafe {
                    let new_len = s.len() - 1;
                    s.as_mut_vec().set_len(new_len);
                }
                ddi::rectangle(
                    self.next_char_pos,
                    Pos::new(CHAR_WIDTH, CHAR_HEIGHT),
                    BACKGROUND_COLOR,
                );
                self.draw_cursor(true);
                continue;
            }
            let char = keyboard::get_char(key);
            if char == 0 {
                continue;
            }
            self.draw_cursor(false);
            let slice = &[char];
            self.print(unsafe { str::from_utf8_unchecked(slice) });
            self.draw_cursor(true);

            unsafe {
                s.as_mut_vec().push(char);
            }

            len += 1;
        }
        self.draw_cursor(false);
        self.newline();
        s
    }
}
