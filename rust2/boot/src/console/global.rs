use alloc::string::String;

use super::console::Console;

static mut CONSOLE: Console = Console::new();

pub fn clear() {
    unsafe {
        (&mut *&raw mut CONSOLE).clear();
    }
}

pub fn print(s: &str) {
    unsafe {
        (&mut *&raw mut CONSOLE).print(s);
    }
}

pub fn newline() {
    unsafe {
        (&mut *&raw mut CONSOLE).newline();
    }
}

pub fn input() -> String {
    unsafe {
        (&mut *&raw mut CONSOLE).input()
    }
}
