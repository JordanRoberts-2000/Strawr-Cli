#[derive(thiserror::Error, Debug)]
pub enum TemplateError {
    #[error("io error: {context}")]
    Io {
        context: String,
        source: std::io::Error,
    },
}
