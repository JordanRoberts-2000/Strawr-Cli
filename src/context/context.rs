use std::path::PathBuf;

use crate::{CliConfig, CliService};

pub struct CliContext {
    pub debug: bool,
    pub storage_dir: PathBuf,
    pub config_path: PathBuf,
    pub config: CliConfig,
    pub service: CliService,
}
