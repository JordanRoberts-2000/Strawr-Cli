use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    state::AppContext,
};

use super::args::GrabCommand;

impl GrabCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), GrabError> {
        let mut manager = GrabManager::new();

        manager.init_storage(ctx)?;
        manager.load_json_data()?;

        let key = match &self.key {
            Some(k) => k.clone(),
            None => manager.select_key()?,
        };

        if self.delete {
            return manager.delete_entry(&key);
        }

        if let Some(ref value) = self.value {
            let encrypt = self
                .encrypt
                .unwrap_or(ctx.config.grab.encrypt_values_by_default);
            return manager.set_entry(&key, value, &encrypt);
        }

        manager.get_entry(&key)?;

        Ok(())
    }
}
