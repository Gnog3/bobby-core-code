use std::collections::HashMap;

use anyhow::{Result, bail};

pub struct Csrs {
    map: HashMap<u32, usize>,
    csrs: Vec<Box<dyn Csr>>,
}

impl Csrs {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            csrs: vec![],
        }
    }

    pub fn insert_csr(&mut self, csr_ids: &[u32], csr: Box<dyn Csr>) {
        let idx = self.csrs.len();
        self.csrs.push(csr);
        for id in csr_ids {
            assert!(self.map.insert(*id, idx).is_none());
        }
    }

    pub fn get_csr(&mut self, csr: u32) -> Result<&mut dyn Csr> {
        let idx = match self.map.get(&csr) {
            Some(idx) => idx,
            None => bail!("No csr found: {}", csr),
        };
        Ok(&mut *self.csrs[*idx])
    }
}

pub trait Csr: Send {
    fn read(&mut self, csr: u32, ram: &mut [u8]) -> Result<u32>;
    fn write(&mut self, csr: u32, ram: &mut [u8], data: u32) -> Result<()>;
}
