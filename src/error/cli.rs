use crate::{commands::errors::*, config::ConfigError, services::errors::*};

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    EditorLaunchFailed(#[from] EditorLauncherError),

    // Commands
    // #[error(transparent)]
    // GrabCommand(#[from] GrabError),
    #[error(transparent)]
    ImgCommand(#[from] ImgError),
    #[error(transparent)]
    SuggestCommand(#[from] SuggestCmdError),
    #[error(transparent)]
    AiCommand(#[from] AiCmdError),
    #[error(transparent)]
    ConfigCommand(#[from] ConfigCommandError),
    // #[error(transparent)]
    // TempCommand(#[from] TempError),
    #[error(transparent)]
    TemplateCommand(#[from] TemplateError),
}
