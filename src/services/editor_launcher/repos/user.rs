use crate::services::editor_launcher::{traits::EditorLauncher, Editor, EditorLauncherError};

use std::{path::Path, process::Command};

pub struct EditorLauncherRepo;

impl EditorLauncher for EditorLauncherRepo {
    fn open(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError> {
        log::info!("Opening {:?} in {:?}", editor, path);

        if !path.exists() {
            return Err(EditorLauncherError::PathDoesntExist(path.to_path_buf()));
        }

        let cmd = match editor {
            Editor::VsCode => "code",
            Editor::Zed => "zed",
            Editor::Vim => "vim",
        };

        if which::which(cmd).is_err() {
            return Err(EditorLauncherError::NotFound(cmd.to_string()));
        }

        let status = match editor {
            Editor::VsCode => Command::new("code").arg(path).arg("-n").status(),
            Editor::Zed => Command::new("zed").arg(path).status(),
            Editor::Vim => Command::new("vim").arg(path).status(),
        };

        match status {
            Ok(s) if s.success() => {
                log::info!("✅ Editor exited successfully.");
                Ok(())
            }
            Ok(s) => Err(EditorLauncherError::NonZeroExit(s.code())),
            Err(e) => Err(EditorLauncherError::LaunchFailed(e)),
        }
    }
}
