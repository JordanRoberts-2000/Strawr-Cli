#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ColorOutput {
    Rgb,
    Hex,
}
