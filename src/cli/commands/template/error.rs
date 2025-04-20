use crate::utils::editor::EditorError;

#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    #[error("io error: {context}")]
    Io {
        context: String,
        source: std::io::Error,
    },
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
    FailedToReadTemplateDir(std::io::Error),
    #[error("Editor failed to open")]
    EditorFailed(#[from] EditorError),
}
