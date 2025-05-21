use inquire::PasswordDisplayMode;

use super::{error::KeyringError, traits::KeyringRepo, UserKeyringRepo};

pub struct KeyringService {
    pub(super) repo: Box<dyn KeyringRepo>,
    pub(super) password_display: PasswordDisplayMode,
    pub(super) service: String,
}

impl KeyringService {
    pub fn new(service: &str) -> Self {
        Self {
            repo: Box::new(UserKeyringRepo),
            password_display: PasswordDisplayMode::Hidden,
            service: service.to_string(),
        }
    }

    pub fn set_password_mode(&mut self, password_display: &PasswordDisplayMode) -> &mut Self {
        self.password_display = *password_display;
        self
    }

    pub fn set_repo(&mut self, repo: impl KeyringRepo + 'static) -> &mut Self {
        self.repo = Box::new(repo);
        self
    }

    pub fn get_or_set(&self, field: impl AsRef<str>) -> Result<String, KeyringError> {
        let field = field.as_ref();
        self.repo
            .get_or_set(&self.service, field, &self.password_display)
    }

    pub fn update(&self, field: impl AsRef<str>) -> Result<(), KeyringError> {
        let field = field.as_ref();
        self.repo
            .update(&self.service, field, &self.password_display)
    }

    pub fn remove(&self, field: impl AsRef<str>) -> Result<(), KeyringError> {
        let field = field.as_ref();
        self.repo.remove(&self.service, field)
    }
}
