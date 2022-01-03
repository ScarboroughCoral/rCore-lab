use core::fmt::Arguments;

use log::{ Log, Level, LevelFilter };

use crate::println;
static LOGGER: SimpleLogger = SimpleLogger;
pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(
        match option_env!("LOG") {
            Some("error") => LevelFilter::Error,
            Some("warn") => LevelFilter::Warn,
            Some("info") => LevelFilter::Info,
            Some("debug") => LevelFilter::Debug,
            Some("trace") => LevelFilter::Trace,
            _ => LevelFilter::Off,
        }
    )
}
struct SimpleLogger;
impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
           return;
        }
        print_with_color(
            format_args!("[{:>5}]:{}", record.level(), record.args()),
            get_log_level_color(record.level())
        )
    }

    fn flush(&self) {
        todo!()
    }
}

fn print_with_color(content: Arguments, color: usize) {
    println!("\x1b[{}m{}\x1b[0m", color,content);
}

fn get_log_level_color(level: Level)-> usize {
    match level {
        Level::Error => 31,
        Level::Warn => 93,
        Level::Info => 34,
        Level::Debug => 32,
        Level::Trace => 90,
    }
}
