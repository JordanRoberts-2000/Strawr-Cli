use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError},
    error::io::IoError,
    utils::fs::{copy_dir_contents, is_dir_empty},
};

impl<'a> TemplateManager<'a> {
    pub fn inject_template_files(
        &self,
        template: &str,
        variant: &Option<String>,
    ) -> Result<(), TemplateError> {
        let variant_str = variant.as_deref().unwrap_or("default");
        let source_path = self.templates_path.join(template).join(variant_str);

        if !source_path.exists() {
            return if variant.is_none() {
                Err(TemplateError::TemplateNotFound(template.to_string()))
            } else {
                Err(TemplateError::VariantNotFound {
                    template: template.to_string(),
                    variant: variant.clone().unwrap_or_default(),
                })
            };
        }

        let current_dir = std::env::current_dir().map_err(IoError::GetCurrentDir)?;
        let is_empty =
            is_dir_empty(&current_dir).map_err(|e| IoError::ReadDir(e, current_dir.clone()))?;

        let should_copy = if is_empty {
            true
        } else {
            self.ctx.input.confirm(
                "Current workspace is not empty, would you like to inject template anyway?",
            )?
        };

        if should_copy {
            copy_dir_contents(&source_path, &current_dir)
                .map_err(|e| IoError::CopyDirContents(e, source_path, current_dir))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{self, File};

    use tempfile::tempdir;

    use crate::{
        cli::commands::template::{test_utils::test_utils::create_test_manager, TemplateError},
        utils::input::Input,
    };

    #[test]
    fn test_inject_template_success() {
        use std::fs::{self, File};
        use std::io::Write;
        use tempfile::tempdir;

        let manager = create_test_manager(Vec::new());

        let template_path = manager
            .ctx
            .storage_dir
            .join("templates")
            .join("react")
            .join("default");

        fs::create_dir_all(&template_path).expect("Failed to create template directory structure");

        let sample_file_path = template_path.join("hello.txt");
        let mut file =
            File::create(&sample_file_path).expect("Failed to create sample template file");
        writeln!(file, "Hello, world!").expect("Failed to write content to sample file");

        let tmp_dir = tempdir().expect("Failed to create temporary directory");
        std::env::set_current_dir(&tmp_dir)
            .expect("Failed to set current working directory to temp dir");

        let result = manager.inject_template_files("react", &None);

        assert!(
            result.is_ok(),
            "inject_template_files returned an error: {result:?}"
        );

        let injected_file_path = tmp_dir.path().join("hello.txt");
        assert!(
            injected_file_path.exists(),
            "Injected file was not found in target directory"
        );
    }

    #[test]
    fn test_returns_template_not_found_when_missing() {
        let manager = create_test_manager(vec![]);

        // Intentionally do NOT create any template files/directories

        let result = manager.inject_template_files("missing-template", &None);

        assert!(matches!(
            result,
            Err(TemplateError::TemplateNotFound { .. })
        ));
    }

    #[test]
    fn test_returns_variant_not_found_when_variant_missing() {
        let manager = create_test_manager(vec![]);

        // Only create the template root directory (without the variant)
        let template_path = manager.ctx.storage_dir.join("templates").join("react");

        std::fs::create_dir_all(&template_path).expect("Failed to create base template directory");

        let result = manager.inject_template_files("react", &Some("missing-variant".to_string()));

        assert!(matches!(result, Err(TemplateError::VariantNotFound { .. })));
    }

    #[test]
    fn test_does_not_inject_when_user_declines() {
        let manager = create_test_manager(vec![Input::Confirm(false)]);

        let template_path = manager
            .ctx
            .storage_dir
            .join("templates")
            .join("react")
            .join("default");

        // Setup template directory and file
        fs::create_dir_all(&template_path).expect("Failed to create template directory structure");
        fs::write(template_path.join("hello.txt"), "Hello, world!")
            .expect("Failed to write template file");

        // Setup non-empty working directory
        let tmp_dir = tempdir().expect("Failed to create temp target dir");
        std::env::set_current_dir(&tmp_dir).expect("Failed to set CWD");
        File::create(tmp_dir.path().join("existing.txt"))
            .expect("Failed to create dummy file to make CWD non-empty");

        let result = manager.inject_template_files("react", &None);
        assert!(result.is_ok(), "inject_template_files failed: {result:?}");

        // File should NOT be injected
        assert!(
            !tmp_dir.path().join("hello.txt").exists(),
            "Template file was injected despite user declining"
        );
    }

    #[test]
    fn test_injects_when_user_accepts() {
        let manager = create_test_manager(vec![Input::Confirm(true)]);

        let template_path = manager
            .ctx
            .storage_dir
            .join("templates")
            .join("react")
            .join("default");

        // Setup template directory and file
        fs::create_dir_all(&template_path).expect("Failed to create template directory structure");
        fs::write(template_path.join("hello.txt"), "Hello, world!")
            .expect("Failed to write template file");

        // Setup non-empty working directory
        let tmp_dir = tempdir().expect("Failed to create temp target dir");
        std::env::set_current_dir(&tmp_dir).expect("Failed to set CWD");
        File::create(tmp_dir.path().join("existing.txt"))
            .expect("Failed to create dummy file to make CWD non-empty");

        let result = manager.inject_template_files("react", &None);
        assert!(result.is_ok(), "inject_template_files failed: {result:?}");

        // File SHOULD be injected
        let injected_file = tmp_dir.path().join("hello.txt");
        assert!(
            injected_file.exists(),
            "Template file was not injected despite user accepting"
        );
    }
}
