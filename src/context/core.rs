use std::path::PathBuf;

use crate::{
    utils::{editor::EditorLauncher, input::CliInput},
    CliConfig,
};

pub struct CliContext {
    pub debug: bool,
    pub storage_dir: PathBuf,
    pub config: CliConfig,
    pub service: CliService,
}

pub struct CliService {
    pub input: Box<dyn CliInput>,
    pub editor_launcher: Box<dyn EditorLauncher>,
}
