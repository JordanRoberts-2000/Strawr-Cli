mod launcher {
    pub(crate) mod cli;
    pub(crate) mod test;
}
mod enums;
pub mod error;
pub(crate) mod traits;

pub(crate) use {enums::Editor, error::EditorLauncherError, launcher::cli::CliEditorLauncher};
