use crate::{
    cli::commands::{grab::GrabError, img::ImgError, temp::TempError, template::TemplateError},
    config::error::ConfigError,
    state::error::StateError,
};

pub mod io;
pub mod parse;
pub mod utils;

pub use io::IoError;
pub use parse::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    State(#[from] StateError),
    #[error(transparent)]
    Config(#[from] ConfigError),

    // Commands
    #[error(transparent)]
    GrabCommand(#[from] GrabError),
    #[error(transparent)]
    ImgCommand(#[from] ImgError),
    #[error(transparent)]
    TempCommand(#[from] TempError),
    #[error(transparent)]
    TemplateCommand(#[from] TemplateError),
}
