use core::fmt;

use blib::{
    ddi::{self, Pos},
    term,
};
use include_bytes_aligned::include_bytes_aligned;

use crate::image::draw_image_at;

const START_X: u16 = 97;
const START_Y: u16 = 145;
const MAX_CHARS_ON_LINE: u16 = 40;
const ERROR_BOI: &[u8] = include_bytes_aligned!(4, "../error-boi.bin");

pub fn start_panic() {
    ddi::floodfill(0x1D7CEA);
    term::set_color(0xFFFFFF);
}

pub fn end_panic() {
    ddi::rectangle(Pos::new(400, START_Y), Pos::new(143, 190), 0xFFFFFF);
    draw_image_at(Pos::new(400, START_Y), ERROR_BOI);
}

pub struct PanicScreenWriter {
    chars_on_current_line: u16,
    lines: u16,
}

impl PanicScreenWriter {
    pub fn new() -> Self {
        Self {
            chars_on_current_line: 0,
            lines: 0,
        }
    }

    pub fn write_u8(&mut self, mut slice: &[u8]) {
        while !slice.is_empty() {
            let to_write = slice
                .len()
                .min((MAX_CHARS_ON_LINE - self.chars_on_current_line) as _);
            term::print_slice(
                Pos {
                    x: self.chars_on_current_line * 6 + START_X,
                    y: self.lines * 8 + START_Y,
                },
                &slice[0..to_write],
            );
            self.chars_on_current_line += to_write as u16;
            if self.chars_on_current_line == MAX_CHARS_ON_LINE {
                self.newline();
            }
            slice = &slice[to_write..];
        }
    }

    pub fn newline(&mut self) {
        self.chars_on_current_line = 0;
        self.lines += 1;
    }
}

impl fmt::Write for PanicScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if bytes[len - 1] == b'\n' {
            if len > 1 {
                self.write_u8(&bytes[0..len - 1]);
            }
            self.newline();
        } else {
            if !s.is_empty() {
                self.write_u8(s.as_bytes());
            }
        }
        Ok(())
    }
}
