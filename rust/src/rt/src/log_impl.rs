use log::{Metadata, Record};

use crate::println_err;

pub struct SimpleLogger;

pub static LOGGER: SimpleLogger = SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println_err!("{} {}", record.level(), record.args());
    }

    fn flush(&self) {}
}
