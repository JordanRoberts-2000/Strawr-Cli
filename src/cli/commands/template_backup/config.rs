#[derive(Debug, serde::Deserialize, Clone)]
pub struct TemplateConfig {
    pub backend_folder_title: String,
    pub frontend_folder_title: String,
}
