use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

pub fn add_import_to_node(import: &str) -> io::Result<()> {
    // Define the possible target file names
    let target_files = vec![
        "index.js",
        "app.js",
        "server.js",
        "main.js",
        "index.ts",
        "app.ts",
        "server.ts",
        "main.ts",
    ];

    // Check both the current directory and the src directory
    let search_dirs = vec![PathBuf::from("."), PathBuf::from("src")];

    for dir in search_dirs {
        for file_name in &target_files {
            let file_path = dir.join(file_name);
            if file_path.exists() {
                println!("Found file: {:?}", file_path);
                add_import(&file_path, import)?;
            }
        }
    }

    Ok(())
}

fn add_import(file_path: &Path, import: &str) -> io::Result<()> {
    // Read the file content
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut has_import = false;
    let mut last_import_index = None;
    let mut lines: Vec<String> = Vec::new();

    // Check each line to see if the provided import is already in the file
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(import) {
            has_import = true;
        }
        if line.starts_with("import ") {
            last_import_index = Some(index); // Track the index of the last import line
        }
        lines.push(line);
    }

    // If the import is already present, exit early
    if has_import {
        println!("{} is already imported in {:?}", import, file_path);
        return Ok(());
    }

    // Prepare the line to add
    let import_line = import.to_string();
    let empty_line = "".to_string();

    // Insert the import after the last import or at the top if there are no imports
    if let Some(import_index) = last_import_index {
        println!("Adding {} after the last import in {:?}", import, file_path);
        lines.insert(import_index + 1, import_line);
    } else {
        println!("Adding {} at the top of {:?}", import, file_path);
        lines.insert(0, import_line);
        lines.insert(1, empty_line); // Add space after import
    }

    // Write the modified content back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    for line in &lines {
        writeln!(file, "{}", line)?;
    }

    println!("{} added to {:?}", import, file_path);
    Ok(())
}
