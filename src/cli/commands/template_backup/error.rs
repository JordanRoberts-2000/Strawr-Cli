use crate::{error::IoError, services::errors::*};

#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    Prompt(#[from] PromptError),

    #[error(transparent)]
    EditorLauncher(#[from] EditorLauncherError),

    #[error("Template '{0}' does not exist")]
    TemplateNotFound(String),

    #[error("Variant '{variant}' does not exist for '{template}'")]
    VariantNotFound { template: String, variant: String },

    #[error("Cannot create template '{0}' as it already exists")]
    TemplateAlreadyExists(String),

    #[error("Cannot create template variant '{0}' as it already exists")]
    VariantAlreadyExists(String),

    #[error("Failed to select")]
    SelectFailed(#[from] inquire::InquireError),

    #[error("No templates set")]
    NoTemplatesExist,

    #[error("No variants exist for template '{0}'")]
    NoVariants(String),

    #[error("validation failed: {0}")]
    Validation(String),
}
