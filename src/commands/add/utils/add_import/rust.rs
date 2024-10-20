use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

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
