use std::{collections::HashMap, path::PathBuf};

pub mod entry;
pub mod storage;

pub struct GrabManager {
    pub json_file_path: PathBuf,
    pub data_map: HashMap<String, String>,
}

impl GrabManager {
    pub fn new() -> Self {
        Self {
            json_file_path: PathBuf::new(),
            data_map: HashMap::new(),
        }
    }
}
