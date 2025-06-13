use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use anyhow::{Result, bail};

use crate::csrs::Csr;

pub struct Heap {
    value: Arc<AtomicU32>,
}

impl Heap {
    pub fn new(value: Arc<AtomicU32>) -> Self {
        Self { value }
    }

    pub fn read(&self) -> u32 {
        self.value.load(Ordering::SeqCst)
    }
}

pub struct HeapCsr {
    value: Arc<AtomicU32>,
}

impl HeapCsr {
    pub fn new(value: Arc<AtomicU32>) -> Self {
        Self { value }
    }
}

impl Csr for HeapCsr {
    fn read(&mut self, _csr: u32, _ram: &mut [u8]) -> Result<u32> {
        bail!("Can't read heap")
    }

    fn write(&mut self, _csr: u32, _ram: &mut [u8], data: u32) -> Result<()> {
        self.value.store(data, Ordering::SeqCst);
        Ok(())
    }
}
