use colored::*;
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init_logger(debug: bool) {
    let mut builder = Builder::new();
    builder.filter_level(if debug {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    });

    builder.format(|buf, record| {
        let message = format!("[{:?}] {}", record.level(), record.args());
        let colored_message = match record.level() {
            Level::Error => message.red(),
            Level::Warn => message.yellow(),
            Level::Info => message.green(),
            Level::Debug => message.blue(),
            Level::Trace => message.normal(),
        };
        writeln!(buf, "{}", colored_message)
    });

    builder.init();
}
