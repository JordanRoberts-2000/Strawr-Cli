use colored::*;
use env_logger::Builder;
use log::{Level, LevelFilter, Record};
use std::{env, io::Write};

pub fn initialize(debug: bool) {
    if !debug {
        return;
    }
    let mut builder = Builder::new();

    if env::var("RUST_LOG").is_err() {
        builder.filter_level(LevelFilter::Debug);
    } else {
        builder.parse_env("RUST_LOG");
    }

    builder.format(|buf, record: &Record| {
        let mut message = format!("[{:?}]", record.level());

        #[cfg(debug_assertions)]
        if record.level() == Level::Error {
            if let (Some(file), Some(line)) = (record.file(), record.line()) {
                message.push_str(&format!(" - {}:{} -", file, line));
            }
        }

        message.push_str(&format!(" {}", record.args()));

        let colored_message = match record.level() {
            Level::Error => message.red(),
            Level::Warn => message.yellow(),
            Level::Info => message.green(),
            Level::Debug => message.blue(),
            Level::Trace => message.bright_black(),
        };

        writeln!(buf, "{}", colored_message)
    });

    builder.init();
}
