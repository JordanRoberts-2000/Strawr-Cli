use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    utils,
};

impl GrabManager {
    pub fn select_key(&self) -> Result<String, GrabError> {
        if self.data_map.is_empty() {
            return Err(GrabError::NoKeysAvailable);
        }

        let keys: Vec<&str> = self.data_map.keys().map(|k| k.as_str()).collect();
        let key = utils::select(keys, "Select key:").prompt()?;

        Ok(key.to_string())
    }
}
