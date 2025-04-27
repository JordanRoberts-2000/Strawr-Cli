use crate::cli::commands::grab::{command::execute::GrabInput, GrabError, GrabManager};

impl GrabManager {
    pub fn select_key(&self, input: &impl GrabInput) -> Result<String, GrabError> {
        if self.data_map.is_empty() {
            return Err(GrabError::NoKeysAvailable);
        }

        let keys: Vec<String> = self.data_map.keys().map(|k| k.to_string()).collect();
        let key = input.select(&keys, "Select key:")?;

        Ok(key.to_string())
    }
}
