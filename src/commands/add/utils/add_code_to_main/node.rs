use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

pub fn add_code_to_node(code: &str) -> io::Result<()> {
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
                add_code(&file_path, code)?;
            }
        }
    }

    Ok(())
}

fn add_code(file_path: &Path, code: &str) -> io::Result<()> {
    // Read the file content
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut has_code = false;
    let mut last_import_index = None;
    let mut lines: Vec<String> = Vec::new();

    // Check each line to see if the provided code is already present in the file
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(code) {
            has_code = true;
        }
        if line.starts_with("import ") {
            last_import_index = Some(index); // Track the index of the last import line
        }
        lines.push(line);
    }

    // If the code is already present, exit early
    if has_code {
        println!("The code is already present in {:?}", file_path);
        return Ok(());
    }

    // Prepare the lines to add
    let config_line = code.to_string();
    let empty_line = "".to_string();

    // Insert the provided code after the last import or at the top if there are no imports
    if let Some(import_index) = last_import_index {
        println!("Adding code after the last import in {:?}", file_path);
        lines.insert(import_index + 2, config_line);
        lines.insert(import_index + 3, empty_line); // Add space after config
    } else {
        println!("Adding code at the top of {:?}", file_path);
        lines.insert(0, config_line);
        lines.insert(1, empty_line); // Add space after config
    }

    // Write the modified content back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    for line in &lines {
        writeln!(file, "{}", line)?;
    }

    println!("Code added to {:?}", file_path);
    Ok(())
}
