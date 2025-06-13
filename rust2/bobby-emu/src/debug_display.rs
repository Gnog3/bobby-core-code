use std::{
    collections::VecDeque,
    sync::mpsc::{Receiver, Sender},
};

use anyhow::{Result, bail};
use ratatui::{
    buffer::Cell,
    prelude::{Buffer, Rect},
    style::Style,
    widgets::Widget,
};

use crate::csrs::Csr;

pub enum DebugDisplayMessage {
    S(String),
    Clear,
}

pub struct DebugDisplay {
    lines: VecDeque<String>,
    recv: Receiver<DebugDisplayMessage>,
}

impl DebugDisplay {
    pub fn new(recv: Receiver<DebugDisplayMessage>) -> Self {
        Self {
            lines: VecDeque::from_iter(std::iter::once("".into())),
            recv,
        }
    }

    fn clear(&mut self) {
        self.lines = VecDeque::from_iter(std::iter::once("".into()));
    }

    fn push_str(&mut self, s: &str) {
        for c in s.chars() {
            if c != '\n' {
                self.lines.back_mut().unwrap().push(c);
            } else {
                self.lines.push_back("".into());
            }
        }

        while self.lines.len() > 100 {
            self.lines.pop_front();
        }
    }

    pub fn update(&mut self) {
        while let Ok(m) = self.recv.try_recv() {
            match m {
                DebugDisplayMessage::S(s) => self.push_str(&s),
                DebugDisplayMessage::Clear => self.clear(),
            }
        }
    }
}

impl Widget for &DebugDisplay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut v = vec![];
        for line in self.lines.iter() {
            for line in line.as_bytes().chunks(area.width as _) {
                v.push(str::from_utf8(line).unwrap());
            }
        }

        let len = v.len();
        for (i, s) in v.into_iter().enumerate() {
            buf.set_string(area.x, area.y + area.height - 1, s, Style::default());
            if i != len - 1 {
                scroll_up(area, buf);
            }
        }
    }
}

fn scroll_up(area: Rect, buf: &mut Buffer) {
    for y in area.y..area.y + area.height - 1 {
        for x in area.x..area.x + area.width {
            let r = buf.cell((x, y + 1)).unwrap();
            *buf.cell_mut((x, y)).unwrap() = r.clone();
        }
    }

    for x in area.x..area.x + area.width {
        *buf.cell_mut((x, area.y + area.height - 1)).unwrap() = Cell::EMPTY;
    }
}

pub struct DebugDisplayCsr {
    send: Sender<DebugDisplayMessage>,
    length: usize,
}

impl DebugDisplayCsr {
    pub fn new(send: Sender<DebugDisplayMessage>) -> Self {
        Self { send, length: 0 }
    }

    pub fn print(&mut self, ram: &mut [u8], addr: usize) {
        let data = &ram[addr..addr + self.length];
        let data = str::from_utf8(data).unwrap();
        self.send.send(DebugDisplayMessage::S(data.into())).unwrap();
    }

    pub fn newline(&mut self) {
        self.send.send(DebugDisplayMessage::S("\n".into())).unwrap();
    }

    pub fn length(&mut self, data: u32) {
        self.length = data as usize;
    }

    pub fn clear(&mut self) {
        self.send.send(DebugDisplayMessage::Clear).unwrap();
    }
}

impl Csr for DebugDisplayCsr {
    fn read(&mut self, _csr: u32, _ram: &mut [u8]) -> Result<u32> {
        bail!("No reading from debug display");
    }

    fn write(&mut self, csr: u32, ram: &mut [u8], data: u32) -> Result<()> {
        match csr {
            1100 => self.print(ram, data as usize),
            1101 => self.newline(),
            1102 => self.length(data),
            1103 => self.clear(),
            _ => unreachable!(),
        }
        Ok(())
    }
}
