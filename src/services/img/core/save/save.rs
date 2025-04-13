use std::path::Path;

use crate::services::img::ImgError;
use crate::services::img::{core::ImgSrc, error::Result, Img};
use crate::utils;

impl Img {
    pub fn save(&self) -> Result<()> {
        if let ImgSrc::Local { path } = &self.src {
            if *path != self.target_path {
                utils::trash(&path).map_err(|e| ImgError::Io {
                    context: format!("failed to delete '{:?}'", path),
                    source: e,
                })?;
            }
        }

        let parent = self.target_path.parent().unwrap_or_else(|| Path::new("."));
        self.save_to(&parent)?;

        Ok(())
    }
}

#[test]
fn test_img_save_replaces_original_file() {
    use std::fs;
    use std::path::PathBuf;

    // Setup
    let output_dir = PathBuf::from("tests/tmp_save");
    let source_path = PathBuf::from("tests/tmp_save/input.png");
    let target_path = PathBuf::from("tests/tmp_save/output.png");

    // Ensure clean state
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("Failed to clear test output folder");
    }
    fs::create_dir_all(&output_dir).expect("Failed to create tmp dir");

    // Copy image to source
    fs::copy("tests/assets/test.png", &source_path).expect("Failed to copy test image");

    // Load image from source
    let mut img = Img::open(&source_path).expect("Should open image");

    // Change the target path
    img.target_path = target_path.clone();

    // Save it (this should delete the input file and write to the target)
    img.save().expect("Save should succeed");

    // Assert that original was deleted
    assert!(
        !source_path.exists(),
        "Original source file should be removed"
    );

    // Assert that the image was saved at the new location
    assert!(
        target_path.exists(),
        "Target path should contain the saved image"
    );

    // Clean up
    fs::remove_dir_all(&output_dir).expect("Failed to clear test output folder");
}
