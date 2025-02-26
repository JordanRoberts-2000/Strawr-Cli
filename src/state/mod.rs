use std::path::PathBuf;

pub mod get_storage_dir;

pub struct AppContext {
    pub debug: bool,
    pub storage_dir: PathBuf,
}

impl AppContext {
    pub fn new(debug: bool, storage_dir: PathBuf) -> Self {
        Self { debug, storage_dir }
    }

    pub fn initialize(debug: &bool) -> Result<AppContext, Box<dyn std::error::Error>> {
        let storage_dir = AppContext::get_storage_dir()?;

        // let settings: Settings = AppContext::get_settings()?;

        Ok(AppContext::new(*debug, storage_dir))
    }
}
