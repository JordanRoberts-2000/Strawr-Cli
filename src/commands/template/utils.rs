use std::{env, fs, io};

use crate::utils::copy_all;

pub fn copy_template(template_name: &str) -> io::Result<()> {
    let project_root = env!("CARGO_MANIFEST_DIR"); // Compile-time project root
    let template_path = format!("{}/assets/templates/{}", project_root, template_name);
    let current_dir = env::current_dir()?; // Get the current directory where the CLI is executed

    if !fs::metadata(&template_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Template path not found: {}", template_path),
        ));
    }
    copy_all(&template_path, &current_dir)?;

    Ok(())
}
