use crate::{commands::errors::*, config::ConfigError, context::ContextError};

mod io;
mod parse;
pub(crate) mod utils;

pub(crate) use io::IoError;
pub(crate) use parse::ParseError;

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    State(#[from] ContextError),
    #[error(transparent)]
    Config(#[from] ConfigError),

    // Commands
    // #[error(transparent)]
    // GrabCommand(#[from] GrabError),
    // #[error(transparent)]
    // ImgCommand(#[from] ImgError),
    // #[error(transparent)]
    // TempCommand(#[from] TempError),
    #[error(transparent)]
    TemplateCommand(#[from] TemplateError),
}
