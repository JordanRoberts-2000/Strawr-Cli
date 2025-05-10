#[derive(Debug, Clone, clap::ValueEnum, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Editor {
    #[clap(name = "vscode", alias = "vs-code", alias = "vs", alias = "v")]
    VsCode,
    #[clap(name = "zed", alias = "z")]
    Zed,
    #[clap(name = "vim", alias = "vi")]
    Vim,
}
