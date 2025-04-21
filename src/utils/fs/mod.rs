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

pub fn is_dir_empty<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let path = path.as_ref();

    let mut entries = fs::read_dir(path)?;
    Ok(entries.next().is_none())
}

pub fn copy_dir_contents(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        // Create directories that don't exist
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let dst_path = dst.join(file_name);

            if entry_path.is_dir() {
                std::fs::create_dir_all(&dst_path)?;
                copy_dir_contents(&entry_path, &dst_path)?;
            } else {
                std::fs::copy(&entry_path, &dst_path)?;
            }
        }
    }

    Ok(())
}
