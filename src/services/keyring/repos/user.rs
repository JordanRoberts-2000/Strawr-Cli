use crate::services::keyring::{
    delete_keyring, keyring, traits::KeyringRepo, update_keyring, KeyringError,
};

pub struct UserKeyringRepo;

impl KeyringRepo for UserKeyringRepo {
    fn get_or_set(
        &self,
        service: &str,
        field: &str,
        password_display: &inquire::PasswordDisplayMode,
    ) -> Result<String, KeyringError> {
        keyring(service, field, password_display)
    }

    fn update(
        &self,
        service: &str,
        field: &str,
        password_display: &inquire::PasswordDisplayMode,
    ) -> Result<(), KeyringError> {
        update_keyring(service, field, password_display)
    }

    fn remove(&self, service: &str, field: &str) -> Result<(), KeyringError> {
        delete_keyring(service, field)
    }
}
