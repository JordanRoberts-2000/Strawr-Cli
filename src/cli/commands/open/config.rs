#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Editor {
    VSCode,
    Nano,
    Vim,
    Path,
}

#[derive(Debug, Deserialize, Validate)]
pub struct OpenConfig {
    pub editor: Editor,
}
