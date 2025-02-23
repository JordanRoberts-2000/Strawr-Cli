use std::path::PathBuf;

pub struct AppContext {
    pub debug: bool,
    pub config_path: PathBuf,
}

impl AppContext {
    pub fn new(debug: bool, config_path: PathBuf) -> Self {
        Self { debug, config_path }
    }
}
