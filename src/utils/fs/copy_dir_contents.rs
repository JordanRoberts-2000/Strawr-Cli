use std::{fs, io, path::Path};

pub fn copy_dir_contents<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source path {:?} is not a valid directory", src),
        ));
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();

        let file_name = match entry_path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let dst_path = dst.join(file_name);

        if entry_path.is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir_contents(&entry_path, &dst_path)?;
        } else {
            fs::copy(&entry_path, &dst_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{io::Read, path::PathBuf};

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_copy_dir_contents() -> std::io::Result<()> {
        // Create a temporary source directory
        let src_dir = tempdir()?;
        let src_path = src_dir.path();

        // Create some files in the source directory
        let file1_path = src_path.join("file1.txt");
        let file2_path = src_path.join("file2.txt");
        fs::write(&file1_path, "Hello")?;
        fs::write(&file2_path, "World")?;

        // Create a subdirectory with a file
        let subdir_path = src_path.join("subdir");
        fs::create_dir(&subdir_path)?;
        let subfile_path = subdir_path.join("subfile.txt");
        fs::write(&subfile_path, "Subdata")?;

        // Create a temporary destination directory
        let dst_dir = tempdir()?;
        let dst_path = dst_dir.path();

        // Perform the copy
        copy_dir_contents(src_path, dst_path)?;

        // Verify that the files were copied
        let dst_file1 = dst_path.join("file1.txt");
        let dst_file2 = dst_path.join("file2.txt");
        let dst_subfile = dst_path.join("subdir").join("subfile.txt");

        assert!(dst_file1.exists());
        assert!(dst_file2.exists());
        assert!(dst_subfile.exists());

        // read and verify content
        let mut buf = String::new();
        fs::File::open(&dst_file1)?.read_to_string(&mut buf)?;
        assert_eq!(buf, "Hello");

        buf.clear();
        fs::File::open(&dst_file2)?.read_to_string(&mut buf)?;
        assert_eq!(buf, "World");

        buf.clear();
        fs::File::open(&dst_subfile)?.read_to_string(&mut buf)?;
        assert_eq!(buf, "Subdata");

        Ok(())
    }

    #[test]
    fn test_copy_overwrites_existing_files() -> std::io::Result<()> {
        let src_dir = tempdir()?;
        let dst_dir = tempdir()?;

        let file_name = "file.txt";
        let src_file = src_dir.path().join(file_name);
        let dst_file = dst_dir.path().join(file_name);

        fs::write(&src_file, "new content")?;
        fs::write(&dst_file, "old content")?;

        copy_dir_contents(src_dir.path(), dst_dir.path())?;

        let mut content = String::new();
        fs::File::open(&dst_file)?.read_to_string(&mut content)?;

        assert_eq!(content, "new content");

        Ok(())
    }

    #[test]
    fn test_copy_from_empty_directory() -> std::io::Result<()> {
        let src_dir = tempdir()?;
        let dst_dir = tempdir()?;

        copy_dir_contents(src_dir.path(), dst_dir.path())?;

        assert!(fs::read_dir(dst_dir.path())?.next().is_none());

        Ok(())
    }

    #[test]
    fn test_copy_from_nonexistent_directory_should_fail() {
        let nonexistent = PathBuf::from("/does/not/exist");
        let dst_dir = tempdir().unwrap();

        let result = copy_dir_contents(nonexistent, dst_dir.path());

        assert!(result.is_err());
    }
}
