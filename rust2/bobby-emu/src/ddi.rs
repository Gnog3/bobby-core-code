use std::sync::mpsc::Sender;

use anyhow::bail;

use crate::{csrs::Csr, display::DisplayEvent};

pub struct DdiCsr {
    send: Sender<DisplayEvent>,
    matrix_1: u32,
    matrix_2: u32,
    target: u32,
    source: u32,
    size: u32,
    color: u32,
}

impl DdiCsr {
    pub fn new(send: Sender<DisplayEvent>) -> Self {
        Self {
            send,
            matrix_1: 0,
            matrix_2: 0,
            target: 0,
            source: 0,
            size: 0,
            color: 0,
        }
    }
}

impl Csr for DdiCsr {
    fn read(&mut self, _csr: u32, _ram: &mut [u8]) -> anyhow::Result<u32> {
        bail!("No read from ddi");
    }

    fn write(&mut self, csr: u32, _ram: &mut [u8], data: u32) -> anyhow::Result<()> {
        match csr {
            1050 => self.matrix_1 = data,
            1051 => self.matrix_2 = data,
            1052 => self.target = data,
            1053 => self.source = data,
            1054 => self.size = data,
            1055 => self.color = data,
            1056 => self
                .send
                .send(DisplayEvent::Matrix {
                    matrix: ((self.matrix_2 as u64) << 32) | self.matrix_1 as u64,
                    target_x: (self.target & 0xFFFF) as u16,
                    target_y: (self.target >> 16) as u16,
                    color: self.color,
                })
                .unwrap(),
            1057 => self
                .send
                .send(DisplayEvent::Floodfill { color: self.color })
                .unwrap(),
            1058 => bail!("ddi copy not implemented"),
            1059 => self.send.send(DisplayEvent::Rectangle {
                target_x: (self.target & 0xFFFF) as u16,
                target_y: (self.target >> 16) as u16,
                size_x: (self.size & 0xFFFF) as u16,
                size_y: (self.size >> 16) as u16,
                color: self.color,
            }).unwrap(),
            _ => unreachable!(),
        }
        Ok(())
    }
}
