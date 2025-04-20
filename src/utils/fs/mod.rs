use std::{fs, io, path::Path};

pub fn list_subfolders<P: AsRef<Path>>(path: P) -> Result<Vec<String>, io::Error> {
    let path = path.as_ref();
    let entries = fs::read_dir(path)?;
    let mut folder_names: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                folder_names.push(name.to_string());
            }
        }
    }

    Ok(folder_names)
}
