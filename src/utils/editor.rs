use std::{path::Path, process::Command};

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

impl Editor {
    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<(), EditorError> {
        let path = path.as_ref();
        log::info!("Opening {:?} in {:?}", self, path);

        let cmd = match self {
            Editor::VsCode => "code",
            Editor::Zed => "zed",
            Editor::Vim => "vim",
        };

        if which::which(cmd).is_err() {
            return Err(EditorError::NotFound(cmd.to_string()));
        }

        let status = match self {
            Editor::VsCode => Command::new("code").arg(path).arg("-n").status(),
            Editor::Zed => Command::new("zed").arg(path).status(),
            Editor::Vim => Command::new("vim").arg(path).status(),
        };

        match status {
            Ok(s) if s.success() => {
                log::info!("✅ Editor exited successfully.");
                Ok(())
            }
            Ok(s) => Err(EditorError::NonZeroExit(s.code())),
            Err(e) => Err(EditorError::LaunchFailed(e)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EditorError {
    #[error("editor exited with non-zero status: {0:?}")]
    NonZeroExit(Option<i32>), // Option because the status code might not exist on all platforms

    #[error("failed to launch editor: {0}")]
    LaunchFailed(#[from] std::io::Error),

    #[error("editor command not found in PATH: {0}")]
    NotFound(String),
}
