use std::{collections::HashMap, path::PathBuf};

pub mod delete_entry;
pub mod get_entry;
pub mod initialize_data_folder;
pub mod open_list_file;
pub mod set_entry;

pub struct GrabService {
    pub json_file_path: PathBuf,
    pub list_file_path: PathBuf,
    pub data_map: HashMap<String, String>,
}

impl GrabService {
    pub fn new() -> Self {
        Self {
            json_file_path: PathBuf::new(),
            list_file_path: PathBuf::new(),
            data_map: HashMap::new(),
        }
    }
}
