pub mod character_printer;
pub mod cpu;
pub mod cpu_thread;
pub mod csrs;
pub mod ddi;
pub mod debug_display;
pub mod display;
pub mod gui;
pub mod heap;
pub mod instruction_formats;
pub mod memory;
pub mod keyboard;

use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU32, Ordering},
    mpsc::channel,
};

use character_printer::CharacterPrinterCsr;
use cpu::Cpu;
use csrs::Csrs;
use ddi::DdiCsr;
use debug_display::{DebugDisplay, DebugDisplayCsr};
use gui::Gui;
use heap::{Heap, HeapCsr};

use clap::Parser;
use keyboard::KeyboardCsr;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    persist_ram: Option<String>,
    #[arg(long)]
    flash: Option<String>,
}

fn main() {
    let mut csrs = Csrs::new();

    // Debug display
    let debug_display = {
        let (send, recv) = channel();
        let debug_display = DebugDisplay::new(recv);
        let debug_display_csr = DebugDisplayCsr::new(send);
        csrs.insert_csr(&[1100, 1101, 1102, 1103], Box::new(debug_display_csr));
        debug_display
    };

    // Heap
    let heap = {
        let value = Arc::new(AtomicU32::new(0));
        let heap = Heap::new(Arc::clone(&value));
        let heap_csr = HeapCsr::new(value);
        csrs.insert_csr(&[1112], Box::new(heap_csr));
        heap
    };

    // Display (ddi, character)
    let display = {
        let (send, recv) = channel();
        let ddi = DdiCsr::new(send.clone());
        csrs.insert_csr(
            &[1050, 1051, 1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059],
            Box::new(ddi),
        );
        let c = CharacterPrinterCsr::new(send);
        csrs.insert_csr(&[1024, 1025, 1026, 1037], Box::new(c));
        recv
    };

    // Keyboard
    let keyboard = {
        let (send, recv) = channel();
        let keyboard = KeyboardCsr::new(recv);
        csrs.insert_csr(&[1035], Box::new(keyboard));
        send
    };

    let args = Args::parse();
    let mut cpu = Cpu::new(csrs);

    if let Some(flash) = args.flash {
        let data = std::fs::read(flash).unwrap();
        cpu.flash(&data);
    }

    let cpu = Arc::new(Mutex::new(cpu));
    let cpu_handle = cpu_thread::run(Arc::clone(&cpu));

    let gui = Gui {
        debug_display,
        cpu,
        heap,
    };
    let gui_handle = gui::run(gui);
    // gui_handle.join().unwrap();
    display::run(display, keyboard);
    cpu_thread::STOP.store(true, Ordering::SeqCst);
    gui_handle.join().unwrap();
    if let Err(err) = cpu_handle.join().unwrap() {
        println!("Err: {:?}", err);
    }
}
