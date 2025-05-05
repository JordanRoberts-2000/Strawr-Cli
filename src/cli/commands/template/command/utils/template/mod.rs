pub mod create;
pub mod has_variants;
pub mod inject;
pub mod new;

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub path: std::path::PathBuf,
}
