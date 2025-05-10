use std::path::Path;

use super::{enums::Editor, error::EditorLauncherError};

pub trait EditorLauncher: Send + Sync {
    fn open(&self, editor: &Editor, path: &Path) -> Result<(), EditorLauncherError>;
}
