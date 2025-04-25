use std::fs;

use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    error::{IoError, ParseError},
};

impl GrabManager {
    pub fn load_json_data(&mut self) -> Result<(), GrabError> {
        let path = &self.json_file_path;

        let json_data = fs::read_to_string(path).map_err(|e| IoError::ReadFile(e, path.clone()))?;

        self.data_map = serde_json::from_str(&json_data).map_err(|e| ParseError::Json {
            source: e,
            title: format!("{:?}", path),
        })?;

        log::trace!("json data loaded into memory");
        Ok(())
    }
}
