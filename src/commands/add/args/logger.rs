use std::{env, io};

use crate::{
    commands::add::utils::detect_codebase::{detect_codebase, CodeBase},
    utils::{copy_to_client_dir, run_command},
};

pub fn add_logger() {
    if detect_codebase(CodeBase::Node) {
        if let Err(e) = get_initialize_logger_file() {
            panic!("Error getting logger file: {}", e);
        };
        if let Err(e) = run_command("npm i pino") {
            panic!("Error installing pino: {}", e);
        };
        if let Err(e) = run_command("npm i pino-pretty -D") {
            panic!("Error installing pino-pretty: {}", e);
        };
    }
}

fn get_initialize_logger_file() -> io::Result<()> {
    let current_dir = env::current_dir().expect("could not get current dir");
    let src_dir = current_dir.join("src");
    if src_dir.exists() {
        copy_to_client_dir("assets/add/logger.ts", "src/utils")?
    } else {
        copy_to_client_dir("assets/add/logger.ts", "utils")?;
    }
    Ok(())
}
