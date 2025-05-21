use inquire::PasswordDisplayMode;

use super::KeyringError;

pub trait KeyringRepo {
    fn get_or_set(
        &self,
        service: &str,
        field: &str,
        password_display: &PasswordDisplayMode,
    ) -> Result<String, KeyringError>;
    fn remove(&self, service: &str, field: &str) -> Result<(), KeyringError>;
    fn update(
        &self,
        service: &str,
        field: &str,
        password_display: &PasswordDisplayMode,
    ) -> Result<(), KeyringError>;
}
