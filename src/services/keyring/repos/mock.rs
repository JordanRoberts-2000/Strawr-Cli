use crate::services::keyring::{traits::KeyringRepo, KeyringError};
use inquire::PasswordDisplayMode;
use std::{cell::RefCell, collections::HashMap};

#[derive(Default)]
pub struct MockKeyringRepo {
    store: RefCell<HashMap<String, String>>,
}

impl MockKeyringRepo {
    pub fn new() -> Self {
        MockKeyringRepo {
            store: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_entries(entries: Vec<(impl Into<String>, impl Into<String>)>) -> Self {
        let mut m = HashMap::new();

        for entry in entries {
            let (field, value) = entry;
            m.insert(field.into(), value.into());
        }

        MockKeyringRepo {
            store: RefCell::new(m),
        }
    }

    pub fn get_stored(&self, field: &str) -> Option<String> {
        self.store.borrow().get(field).cloned()
    }
}

impl KeyringRepo for MockKeyringRepo {
    fn get_or_set(
        &self,
        _service: &str,
        field: &str,
        _display: &PasswordDisplayMode,
    ) -> Result<String, KeyringError> {
        let store = self.store.borrow_mut();
        if let Some(val) = store.get(field) {
            return Ok(val.clone());
        }
        // Simulate user being prompted and cancelling
        Err(KeyringError::EntryNotFound("mock".into(), field.into()))
    }

    fn update(
        &self,
        _service: &str,
        field: &str,
        _display: &PasswordDisplayMode,
    ) -> Result<(), KeyringError> {
        let mut store = self.store.borrow_mut();
        if store.contains_key(field) {
            store.insert(field.to_string(), "<updated>".into());
            Ok(())
        } else {
            Err(KeyringError::EntryNotFound("mock".into(), field.into()))
        }
    }

    fn remove(&self, _service: &str, field: &str) -> Result<(), KeyringError> {
        let mut store = self.store.borrow_mut();
        if store.remove(field).is_some() {
            Ok(())
        } else {
            Err(KeyringError::EntryNotFound("mock".into(), field.into()))
        }
    }
}
