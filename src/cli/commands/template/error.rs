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

    #[error("Cannot create template as it already exists")]
    TemplateAlreadyExists,

    #[error("Cannot create template variant as it already exists")]
    VariantAlreadyExists,

    #[error("Attempted to create a variant of a non-existent template")]
    CreatingVariantWithoutDefault,

    #[error("Attempted to create a variant of a non-existent template")]
    NoExistingTemplate(std::io::Error),

    #[error("Editor failed to open")]
    EditorFailed(#[from] EditorError),

    #[error("Failed to select")]
    SelectFailed(#[from] inquire::InquireError),

    #[error("No templates set")]
    NoTemplates,
}
