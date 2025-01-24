use std::{
    env,
    fs::{self, File},
    io::{self, Error, ErrorKind, Write},
    path::{Path, PathBuf},
    process::Command,
};

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

pub fn copy_to_client_dir(path: &str, client_path: &str) -> io::Result<()> {
    let client_dir = env::current_dir()?;
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let from_path = project_root.join(path);

    if !from_path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Source path does not exist",
        ));
    }

    let client_dest_path = client_dir.join(client_path);

    if client_dest_path.extension().is_some() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Client path should not have an extension",
        ));
    }

    fs::create_dir_all(&client_dest_path)?;
    copy_item(&from_path, &client_dest_path)?;

    Ok(())
}

pub fn copy_to_client(path: &str) -> io::Result<()> {
    let client_dir = env::current_dir()?;
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let from_path = project_root.join(path);

    if !from_path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "Source path does not exist",
        ));
    }

    copy_item(&from_path, &client_dir)?;

    Ok(())
}

pub fn create_file(filename: &str, content: &str) -> io::Result<()> {
    let file_path = Path::new(filename);

    if file_path.exists() {
        println!("File already exists: {:?}", file_path);
        return Ok(());
    }

    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    println!("File created at: {:?}", file_path);
    Ok(())
}

fn copy_item(from_path: &Path, client_dest_path: &Path) -> io::Result<()> {
    if from_path.is_dir() {
        copy_dir(from_path, client_dest_path)?;
    } else {
        let dest_file_path = client_dest_path.join(from_path.file_name().unwrap());
        fs::copy(&from_path, &dest_file_path)?;
        println!("Copied file to: {:?}", dest_file_path);
    }
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(path.file_name().unwrap());

        if path.is_dir() {
            // Skip node_modules directory
            if path.file_name().unwrap() == "node_modules" {
                continue;
            }
            fs::create_dir_all(&dest_path)?;
            copy_dir(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
            println!("Copied file to: {:?}", dest_path);
        }
    }
    Ok(())
}
