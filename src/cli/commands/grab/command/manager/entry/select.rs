use crate::cli::commands::grab::{GrabError, GrabManager};

impl<'a> GrabManager<'a> {
    pub fn select_key(&self) -> Result<String, GrabError> {
        if self.data_map.is_empty() {
            return Err(GrabError::NoKeysAvailable);
        }

        let keys: Vec<String> = self.data_map.keys().map(|k| k.to_string()).collect();
        let key = self.ctx.input.select(&keys, "Select key:")?;

        Ok(key.to_string())
    }
}
