use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

pub enum CodeBase {
    Node,
    React,
    Rust,
}

pub fn detect_codebase(code_base: CodeBase) -> bool {
    match code_base {
        CodeBase::Node => detect_node_project(),
        CodeBase::React => detect_react_project(),
        CodeBase::Rust => detect_rust_project(),
    }
}

fn detect_rust_project() -> bool {
    Path::new("Cargo.toml").exists()
}

fn detect_node_project() -> bool {
    Path::new("package.json").exists()
}

fn detect_react_project() -> bool {
    if let Ok(contents) = fs::read_to_string("package.json") {
        contents.contains("\"react\"") || contents.contains("\"react-dom\"")
    } else {
        false
    }
}

pub fn add_to_gitignore(input: &str) -> io::Result<()> {
    let gitignore_path = Path::new(".gitignore");

    if gitignore_path.exists() {
        let gitignore_content = fs::read_to_string(gitignore_path)?;
        if !gitignore_content.contains(input) {
            let mut file = OpenOptions::new().append(true).open(gitignore_path)?;

            // Ensure the last character of the file is a newline
            if !gitignore_content.ends_with('\n') {
                write!(file, "\n")?;
            }

            writeln!(file, "{}", input)?;
            println!("{input} added to .gitignore");
        } else {
            println!("{input} is already in the .gitignore");
        }
    } else {
        // If .gitignore does not exist, create it with ".env"
        let mut file = fs::File::create(gitignore_path)?;
        writeln!(file, "{}", input)?;
        println!(".gitignore created and {input} added");
    }

    Ok(())
}

pub fn add_import_to_main_rs(import: &str) -> io::Result<()> {
    let main_rs_path = Path::new("src/main.rs");

    if !main_rs_path.exists() {
        println!("main.rs not found in the src directory.");
        return Ok(());
    }

    let file = fs::File::open(&main_rs_path)?;
    let reader = BufReader::new(file);

    let mut has_import = false;

    // Collect the file content line by line
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains(import) {
            has_import = true;
        }
        lines.push(line);
    }

    if has_import {
        println!("{import} is already present in the file.");
        return Ok(());
    }

    // Check if the first line is `fn main()` (no imports), and insert `use dotenv::dotenv;`
    if lines
        .first()
        .map_or(false, |line| line.trim_start().starts_with("fn main()"))
    {
        println!("Inserting {import} before fn main() with an empty line after.");
        // Insert import at the top, followed by an empty line
        lines.insert(0, "".to_string()); // Insert an empty line
        lines.insert(0, import.to_string());
    } else {
        // Always insert import at the top, if not already present
        println!("Inserting use dotenv::dotenv; at the top of the file.");
        lines.insert(0, import.to_string());
    }

    // Write the updated content back to main.rs
    let mut file = fs::File::create(&main_rs_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn add_to_main_fn(input: &str) -> io::Result<()> {
    let main_rs_path = Path::new("src/main.rs");

    if !main_rs_path.exists() {
        println!("main.rs not found in the src directory.");
        return Ok(());
    }

    let file = fs::File::open(&main_rs_path)?;
    let reader = BufReader::new(file);

    let mut has_input_in_main = false;

    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.contains(input) {
            has_input_in_main = true;
        }
        lines.push(line);
    }

    // If dotenv().ok() is already in the main function, exit early
    if has_input_in_main {
        println!("{input} is already present in the main function.");
        return Ok(());
    }

    // Add input at the top of the main function
    let mut modified_lines: Vec<String> = Vec::new();
    let mut main_found = false;
    for line in lines {
        if line.contains("fn main()") {
            main_found = true;
            modified_lines.push(line.clone());
            modified_lines.push(input.to_string());
        } else {
            modified_lines.push(line);
        }
    }

    if main_found {
        println!("Adding {input} to the main function.");

        // Write the updated content back to main.rs
        let mut file = fs::File::create(&main_rs_path)?;
        for line in modified_lines {
            writeln!(file, "{}", line)?;
        }
    } else {
        println!("main function not found in main.rs.");
    }

    Ok(())
}
