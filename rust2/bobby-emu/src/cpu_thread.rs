use std::{
    sync::{
        atomic::{AtomicBool, Ordering}, Arc, Mutex
    },
    thread::JoinHandle, time::Duration,
};

use anyhow::Result;

use crate::cpu::Cpu;

pub static STOP: AtomicBool = AtomicBool::new(false);

pub fn run(cpu: Arc<Mutex<Cpu>>) -> JoinHandle<Result<()>> {
    std::thread::Builder::new()
        .name("cpu".into())
        .spawn(move || {
            loop {
                if STOP.load(Ordering::SeqCst) {
                    return Ok(());
                }
                {
                    let mut cpu = cpu.lock().unwrap();
                    cpu.tick()?;
                }
                std::thread::sleep(Duration::from_micros(5));
                // std::thread::yield_now();
            }
        })
        .unwrap()
}
