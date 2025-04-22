use std::{fs, io, path::Path};

pub fn is_dir_empty<P: AsRef<Path>>(path: P) -> io::Result<bool> {
    let path = path.as_ref();

    let mut entries = fs::read_dir(path)?;
    Ok(entries.next().is_none())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn detects_empty_directory() -> io::Result<()> {
        let dir = tempdir()?;
        let path = dir.path();

        assert_eq!(is_dir_empty(path)?, true);

        Ok(())
    }

    #[test]
    fn detects_non_empty_directory() -> io::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("dummy.txt");

        File::create(&file_path)?;

        assert_eq!(is_dir_empty(dir.path())?, false);

        Ok(())
    }

    #[test]
    fn errors_on_nonexistent_path() {
        let path = Path::new("/this/does/not/exist/1234");
        let result = is_dir_empty(path);
        assert!(result.is_err());
    }
}
