use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ConfigOption {
    #[strum(to_string = "Open configuration file")]
    OpenConfig,
    #[strum(to_string = "Update API key")]
    UpdateApiKey,
    #[strum(to_string = "Update password")]
    UpdatePassword,
}
