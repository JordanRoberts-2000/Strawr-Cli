pub enum Keyring {
    Password,
    OpenAiKey,
}

impl AsRef<str> for Keyring {
    fn as_ref(&self) -> &str {
        match self {
            Keyring::Password => "encryption password",
            Keyring::OpenAiKey => "open-ai api-key",
        }
    }
}
