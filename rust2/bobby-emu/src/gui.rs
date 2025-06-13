use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
    time::Duration,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, poll};
use ratatui::{Frame, text::Text, widgets::Block};

use crate::{cpu::Cpu, debug_display::DebugDisplay, heap::Heap};

pub struct Gui {
    pub debug_display: DebugDisplay,
    pub cpu: Arc<Mutex<Cpu>>,
    pub heap: Heap,
}

pub fn run(gui: Gui) -> JoinHandle<()> {
    std::thread::Builder::new()
        .name("gui".into())
        .spawn(move || thread(gui))
        .unwrap()
}

fn thread(mut gui: Gui) {
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame| {
                registers(frame, &gui);
                debug_display(frame, &mut gui);
            })
            .expect("failed to draw frame");
        if poll(Duration::from_millis(16)).unwrap() {
            let event = event::read().expect("failed to read event");
            match event {
                Event::Key(KeyEvent { code, .. }) => {
                    if code == KeyCode::Esc {
                        break;
                    }
                }
                _ => {}
            }
        }
    }
    ratatui::restore();
}

const WIDTH: u16 = 17;
const REGISTERS_HEIGHT: u16 = 34;

fn registers(frame: &mut Frame<'_>, gui: &Gui) {
    let mut area = frame.area();
    area.width = WIDTH;
    area.height = REGISTERS_HEIGHT;
    let block = Block::bordered().title("Registers");

    let cpu = gui.cpu.lock().unwrap();

    for i in 0..32 {
        let text = Text::raw(format!("x{:<3} 0x{:08X}", i, cpu.read_register(i)));
        let area = {
            let mut area = block.inner(area);
            area.y += i as u16;
            area
        };
        frame.render_widget(text, area);
    }
    frame.render_widget(block, area);

    area.x += WIDTH;
    area.height = 3;
    let block = Block::bordered().title("PC");
    let text = Text::raw(format!("0x{:08X}", cpu.pc)).right_aligned();
    frame.render_widget(text, block.inner(area));
    frame.render_widget(block, area);

    area.y += 3;
    let block = Block::bordered().title("Heap");
    let text = Text::raw(format!("{} bytes", gui.heap.read())).right_aligned();
    frame.render_widget(text, block.inner(area));
    frame.render_widget(block, area);

    area.y += 3;
    let block = Block::bordered().title("Insn count");
    let text = Text::raw(format!("{}", cpu.insn_count)).right_aligned();
    frame.render_widget(text, block.inner(area));
    frame.render_widget(block, area);

    area.y += 3;
    let block = Block::bordered().title("I/s");
    let text = Text::raw(format!("{}", cpu.fps)).right_aligned();
    frame.render_widget(text, block.inner(area));
    frame.render_widget(block, area);
}

fn debug_display(frame: &mut Frame<'_>, gui: &mut Gui) {
    let mut area = frame.area();
    area.x += 2 * WIDTH;
    area.width -= 2 * WIDTH;
    let block = Block::bordered().title("Debug");

    gui.debug_display.update();
    frame.render_widget(&gui.debug_display, block.inner(area));
    frame.render_widget(block, area);
}
