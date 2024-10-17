use std::{fs, io, path::Path, process::Command};

pub fn run_command(command_str: &str) -> io::Result<()> {
    let mut parts = command_str.split_whitespace();
    let command = parts.next().expect("Command cannot be empty");
    let args: Vec<&str> = parts.collect();

    let status = Command::new(command).args(&args).status()?;

    if status.success() {
        println!("Command '{}' executed successfully.", command_str);
    } else {
        println!("Command '{}' failed with status: {:?}", command_str, status);
    }

    Ok(())
}

pub fn copy_all(src: &str, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(path.file_name().unwrap());

        if path.is_dir() {
            if path.file_name().unwrap() == "node_modules" {
                continue;
            }
            fs::create_dir_all(&dest_path)?;
            copy_all(path.to_str().unwrap(), &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
            println!("Copied file to: {:?}", dest_path);
        }
    }
    Ok(())
}
