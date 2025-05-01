use crate::{
    error::io::IoError,
    utils::{editor::EditorError, input::InputError},
};

#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    Input(#[from] InputError),

    #[error("Template '{template}' does not exist")]
    TemplateNotFound { template: String },

    #[error("Variant '{variant}' does not exist for '{template}'")]
    VariantNotFound { template: String, variant: String },

    #[error("Cannot create template '{template}' as it already exists")]
    TemplateAlreadyExists { template: String },

    #[error("Cannot create template variant '{variant}' as it already exists")]
    VariantAlreadyExists { variant: String },

    #[error("Editor failed to open")]
    EditorFailed(#[from] EditorError),

    #[error("Failed to select")]
    SelectFailed(#[from] inquire::InquireError),

    #[error("No templates set")]
    NoTemplates,
}
