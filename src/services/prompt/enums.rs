use inquire::PasswordDisplayMode;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PasswordDisplay {
    Hidden,
    Masked,
    Full,
}

impl From<PasswordDisplay> for PasswordDisplayMode {
    fn from(d: PasswordDisplay) -> Self {
        match d {
            PasswordDisplay::Hidden => PasswordDisplayMode::Hidden,
            PasswordDisplay::Masked => PasswordDisplayMode::Masked,
            PasswordDisplay::Full => PasswordDisplayMode::Full,
        }
    }
}
