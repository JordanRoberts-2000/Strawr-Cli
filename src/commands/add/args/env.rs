use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use crate::{
    commands::add::utils::{
        add_import_to_main_rs, add_to_gitignore, add_to_main_fn, detect_codebase, CodeBase,
    },
    utils::{create_file, run_command},
};

pub fn add_env() {
    if let Err(e) = create_file(".env", "") {
        panic!("Error creating .env file: {}", e);
    };
    if detect_codebase(CodeBase::Node) || detect_codebase(CodeBase::React) {
        if let Err(e) = run_command("npm i dotenv") {
            panic!("Error installing dotenv: {}", e);
        };
        if detect_codebase(CodeBase::Node) {
            if let Err(e) = add_dotenv_to_node() {
                panic!("Error adding dotenv to node: {}", e);
            };
        }
    }
    if detect_codebase(CodeBase::Rust) {
        if let Err(e) = run_command("cargo add dotenv") {
            panic!("Error installing dotenv: {}", e);
        };
        if let Err(e) = add_import_to_main_rs("use dotenv::dotenv;") {
            panic!("Error adding dotenv: {}", e);
        };
        let initialize_env =
            "if let Err(err) = dotenv() {\n\tpanic!(\"Error loading .env file: {}\", err);\n}";
        if let Err(e) = add_to_main_fn(initialize_env) {
            panic!("Error adding dotenv: {}", e);
        };
    }
    if let Err(e) = add_to_gitignore(".env") {
        panic!("Error handling git-ignore: {}", e);
    };
}

fn add_dotenv_to_node() -> io::Result<()> {
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
                process_file(&file_path)?;
            }
        }
    }

    Ok(())
}

fn process_file(file_path: &Path) -> io::Result<()> {
    // Read the file content
    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut has_dotenv_import = false;
    let mut has_dotenv_config = false;
    let mut last_import_index = None;
    let mut lines: Vec<String> = Vec::new();

    // Check each line to see if dotenv is already imported and configured
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(r#"import dotenv from "dotenv";"#) {
            has_dotenv_import = true;
        }
        if line.contains("dotenv.config();") {
            has_dotenv_config = true;
        }
        if line.starts_with("import ") {
            last_import_index = Some(index); // Track the index of the last import line
        }
        lines.push(line);
    }

    // If dotenv is already imported and configured, exit early
    if has_dotenv_import && has_dotenv_config {
        println!(
            "dotenv is already imported and configured in {:?}",
            file_path
        );
        return Ok(());
    }

    // Prepare the lines to add
    let import_line = r#"import dotenv from "dotenv";"#.to_string();
    let config_line = "dotenv.config();".to_string();
    let empty_line = "".to_string();

    // Insert the dotenv lines appropriately
    if let Some(import_index) = last_import_index {
        // If there are other imports, insert dotenv after the last import
        println!(
            "Adding dotenv import and config after the last import in {:?}",
            file_path
        );
        lines.insert(import_index + 1, import_line);
        lines.insert(import_index + 2, empty_line);
        lines.insert(import_index + 3, config_line);
        // No additional empty line after dotenv.config(); since it comes after imports
    } else {
        // If no imports, add dotenv at the top of the file with spacing
        println!(
            "Adding dotenv import and config at the top of {:?}",
            file_path
        );
        lines.insert(0, import_line);
        lines.insert(1, empty_line.clone()); // Add space after import
        lines.insert(2, config_line);
        lines.insert(3, empty_line); // Add space after config
    }

    // Write the modified content back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    for line in &lines {
        writeln!(file, "{}", line)?;
    }

    println!("dotenv import and config added to {:?}", file_path);
    Ok(())
}
