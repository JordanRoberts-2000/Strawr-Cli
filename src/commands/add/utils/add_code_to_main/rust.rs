use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

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
