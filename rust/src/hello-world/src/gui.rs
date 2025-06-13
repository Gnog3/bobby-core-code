use alloc::string::String;
use rt::{
    ddi::{self, Pos},
    term,
};

const COLORS_NOT_PRESSED: [u32; 4] = [0xFFFFFF, 0xDEDEDE, 0x848484, 0];
const COLORS_PRESSED: [u32; 4] = [0, 0x848484, 0xDEDEDE, 0xFFFFFF];

pub trait Widget {
    fn draw(&self, offset: Pos);
}

pub struct Button {
    pos: Pos,
    size: Pos,
    is_pressed: bool,
}

impl Button {
    pub const fn new(pos: Pos, size: Pos) -> Self {
        Self {
            pos,
            size,
            is_pressed: false,
        }
    }
}

impl Widget for Button {
    fn draw(&self, offset: Pos) {
        draw_panel(self.pos + offset, self.size, self.is_pressed);
    }
}

pub struct Text {
    text: String,
    color: u32,
    pos: Pos,
}

impl Text {
    pub fn new(text: String, color: u32, pos: Pos) -> Self {
        Self { text, color, pos }
    }

    pub fn from_rect_centered(text: String, color: u32, pos: Pos, size: Pos) {
        // let new_size = size - Pos::new(CH
    }
}

impl Widget for Text {
    fn draw(&self, offset: Pos) {
        let pos = self.pos + offset;
        term::set_color(self.color);
        term::print_slice(pos, self.text.as_bytes());
    }
}

fn draw_panel(pos: Pos, size: Pos, invert_colors: bool) {
    let colors = if invert_colors {
        &COLORS_PRESSED
    } else {
        &COLORS_NOT_PRESSED
    };
    // Draw the top border
    ddi::rectangle(Pos::new(pos.x, pos.y), Pos::new(size.x - 2, 1), colors[0]);

    // Draw the left border
    ddi::rectangle(
        Pos::new(pos.x, pos.y + 1),
        Pos::new(1, size.y - 2),
        colors[0],
    );

    // Draw the inner top and left shadows
    ddi::rectangle(
        Pos::new(pos.x + 1, pos.y + 1),
        Pos::new(size.x - 3, 1),
        colors[1],
    );
    ddi::rectangle(
        Pos::new(pos.x + 1, pos.y + 1),
        Pos::new(1, size.y - 3),
        colors[1],
    );

    // Draw the right border
    ddi::rectangle(
        Pos::new(pos.x + size.x - 1, pos.y),
        Pos::new(1, size.y),
        colors[3],
    );

    // Draw the bottom border
    ddi::rectangle(
        Pos::new(pos.x, pos.y + size.y - 1),
        Pos::new(size.x, 1),
        colors[3],
    );

    // Draw the inner right and bottom shadows
    ddi::rectangle(
        Pos::new(pos.x + size.x - 2, pos.y + 1),
        Pos::new(1, size.y - 2),
        colors[2],
    );
    ddi::rectangle(
        Pos::new(pos.x + 1, pos.y + size.y - 2),
        Pos::new(size.x - 3, 1),
        colors[2],
    );

    // Draw the central fill area
    ddi::rectangle(
        Pos::new(pos.x + 2, pos.y + 2),
        Pos::new(size.x - 4, size.y - 4),
        0xC6C6C6,
    );
}
