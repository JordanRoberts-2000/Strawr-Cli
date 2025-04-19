use colored::*;
use env_logger::Builder;
use log::{Level, LevelFilter, Record};
use std::{env, io::Write};

pub fn initialize(debug: bool) {
    let mut builder = Builder::new();

    if let Ok(rust_log) = env::var("RUST_LOG") {
        builder.parse_filters(&rust_log);
    } else if debug {
        builder.filter_level(LevelFilter::Debug);
    } else {
        builder.filter_level(LevelFilter::Warn);
    }

    builder.format(|buf, record: &Record| {
        let mut message = format!("[{:?}]", record.level());
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
