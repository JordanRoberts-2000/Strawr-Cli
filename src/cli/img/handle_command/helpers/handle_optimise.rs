use std::fs;
use std::path::Path;

pub fn handle_optimization(path: &String, output: &Option<String>) {
    let path = Path::new(&path);

    if path.is_absolute() {
        println!("Absolute path: {}", path.display());
    } else if path.is_relative() {
        println!("Relative path: {}", path.display());
    }

    if let Some(file_path) = find_file_with_any_extension(path) {
        println!("Found file: {}", file_path.display());
        // optimize_file(&file_path, output);
    } else if path.is_dir() {
        println!("Path leads to a folder: {}", path.display());
        // optimize_folder(path, output);
    } else {
        println!("Invalid path or file not found: {}", path.display());
    }
}

fn find_file_with_any_extension(base_path: &Path) -> Option<std::path::PathBuf> {
    if base_path.is_file() {
        return Some(base_path.to_path_buf());
    }

    let dir = base_path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = base_path.file_name()?.to_str()?;

    // Read directory entries and find matching file
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    if name == file_name {
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}
