use std::path::PathBuf;

use crate::{
    config::Config,
    utils::{editor::EditorLauncher, input::CliInput},
};

pub mod error;
pub mod initialize;
pub mod storage_dir;

pub struct AppContext {
    pub debug: bool,
    pub storage_dir: PathBuf,
    pub config: Config,
    pub input: Box<dyn CliInput>,
    pub editor: Box<dyn EditorLauncher>,
}
