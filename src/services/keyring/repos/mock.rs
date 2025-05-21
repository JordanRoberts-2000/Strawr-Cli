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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_or_set_returns_existing_value() {
        let mock = MockKeyringRepo::with_entries(vec![("foo", "bar")]);
        let value = mock
            .get_or_set("ignored_service", "foo", &PasswordDisplayMode::Hidden)
            .unwrap();
        assert_eq!(value, "bar");
        // ensure get_stored still returns the same
        assert_eq!(mock.get_stored("foo").as_deref(), Some("bar"));
    }

    #[test]
    fn get_or_set_missing_returns_entry_not_found() {
        let mock = MockKeyringRepo::new();
        let err = mock
            .get_or_set("svc", "missing", &PasswordDisplayMode::Hidden)
            .unwrap_err();
        match err {
            KeyringError::EntryNotFound(svc, field) => {
                assert_eq!(svc, "mock");
                assert_eq!(field, "missing");
            }
            other => panic!("expected EntryNotFound, got {:?}", other),
        }
    }

    #[test]
    fn update_existing_overwrites_value() {
        let mock = MockKeyringRepo::with_entries(vec![("token", "old")]);
        mock.update("svc", "token", &PasswordDisplayMode::Hidden)
            .unwrap();
        // update should set to "<updated>"
        assert_eq!(mock.get_stored("token").as_deref(), Some("<updated>"));
    }

    #[test]
    fn update_missing_returns_entry_not_found() {
        let mock = MockKeyringRepo::new();
        let err = mock
            .update("svc", "nope", &PasswordDisplayMode::Hidden)
            .unwrap_err();
        match err {
            KeyringError::EntryNotFound(svc, field) => {
                assert_eq!(svc, "mock");
                assert_eq!(field, "nope");
            }
            other => panic!("expected EntryNotFound, got {:?}", other),
        }
    }

    #[test]
    fn remove_existing_deletes_entry() {
        let mock = MockKeyringRepo::with_entries(vec![("key", "value")]);
        mock.remove("svc", "key").unwrap();
        // after removal, get_stored should return None
        assert!(mock.get_stored("key").is_none());
    }

    #[test]
    fn remove_missing_returns_entry_not_found() {
        let mock = MockKeyringRepo::new();
        let err = mock.remove("svc", "absent").unwrap_err();
        match err {
            KeyringError::EntryNotFound(svc, field) => {
                assert_eq!(svc, "mock");
                assert_eq!(field, "absent");
            }
            other => panic!("expected EntryNotFound, got {:?}", other),
        }
    }
}
