use crate::{
    cli::commands::grab::{GrabError, GrabManager},
    state::AppContext,
    utils::input::SelectInput,
};

use super::args::GrabCommand;

pub trait GrabInput: SelectInput {}
impl<T: SelectInput> GrabInput for T {}

impl GrabCommand {
    pub fn execute(&self, ctx: &AppContext, input: &impl GrabInput) -> Result<(), GrabError> {
        let mut manager = GrabManager::new();

        manager.init_storage(ctx)?;
        manager.load_json_data()?;

        let key = match &self.key {
            Some(k) => k.clone(),
            None => manager.select_key(input)?,
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
