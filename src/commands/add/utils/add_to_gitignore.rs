use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

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
