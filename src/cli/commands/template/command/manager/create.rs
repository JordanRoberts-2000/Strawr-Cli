use std::fs;

use crate::{
    cli::commands::template::{command::manager::TemplateManager, TemplateError, DEFAULT_FOLDER},
    error::io::IoError,
};

impl<'a> TemplateManager<'a> {
    pub fn create_template(
        &self,
        template: &str,
        variant: Option<&str>,
    ) -> Result<(), TemplateError> {
        match variant {
            Some(variant) => self.create_template_variant(&template, variant),
            None => self.create_initial_template(&template),
        }?;

        Ok(())
    }

    fn create_initial_template(&self, template: &str) -> Result<(), TemplateError> {
        let template_path = self.templates_path.join(template);

        if template_path.exists() {
            return Err(TemplateError::TemplateAlreadyExists(template.to_string()));
        }

        let default_path = template_path.join(DEFAULT_FOLDER);

        fs::create_dir_all(&default_path)
            .map_err(|e| IoError::CreateDir(e, default_path.clone()))?;
        log::info!("Created new template '{}'", template);

        self.ctx.editor.open(self.editor, &default_path)?;

        Ok(())
    }

    fn create_template_variant(&self, template: &str, variant: &str) -> Result<(), TemplateError> {
        let template_path = self.templates_path.join(template);

        if !template_path.exists() {
            return Err(TemplateError::TemplateNotFound(template.to_string()));
        }

        let variant_path = template_path.join(&variant);

        if variant_path.exists() {
            return Err(TemplateError::VariantAlreadyExists(variant.to_string()));
        }

        fs::create_dir(&variant_path).map_err(|e| IoError::CreateDir(e, variant_path.clone()))?;
        log::info!("Created variant '{variant}' for template '{template}'");

        self.ctx.editor.open(self.editor, &variant_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::template::test_utils::test_utils::create_test_manager;
    use crate::cli::commands::template::TemplateError;

    #[test]
    fn test_create_template_success() {
        let manager = create_test_manager(Vec::new());

        let result = manager.create_template("new_template", None);
        assert!(result.is_ok());

        let expected_path = manager
            .ctx
            .storage_dir
            .join("templates")
            .join("new_template")
            .join(DEFAULT_FOLDER);
        assert!(
            expected_path.exists() && expected_path.is_dir(),
            "Template folder not created"
        );
    }

    #[test]
    fn test_create_template_already_exists() {
        let manager = create_test_manager(Vec::new());

        // First creation
        assert!(manager.create_template("existing_template", None).is_ok());

        // Second creation should fail
        let result = manager.create_template("existing_template", None);
        assert!(matches!(
            result,
            Err(TemplateError::TemplateAlreadyExists { .. })
        ));
    }

    #[test]
    fn test_create_template_variant_success() {
        let manager = create_test_manager(Vec::new());

        // Create base template first
        assert!(manager.create_template("base_template", None).is_ok());

        // Now add a variant
        let result = manager.create_template("base_template", Some("variant1"));
        assert!(result.is_ok());

        let expected_variant_path = manager
            .ctx
            .storage_dir
            .join("templates")
            .join("base_template")
            .join("variant1");
        assert!(expected_variant_path.exists() && expected_variant_path.is_dir());
    }

    #[test]
    fn test_create_template_variant_already_exists() {
        let manager = create_test_manager(Vec::new());

        // Create template and variant
        assert!(manager.create_template("t", None).is_ok());
        assert!(manager.create_template("t", Some("v")).is_ok());

        // Try to create the same variant again
        let result = manager.create_template("t", Some("v"));
        assert!(matches!(
            result,
            Err(TemplateError::VariantAlreadyExists { .. })
        ));
    }

    #[test]
    fn test_create_template_variant_template_not_found() {
        let manager = create_test_manager(Vec::new());

        let result = manager.create_template("nonexistent_template", Some("variant"));
        assert!(matches!(
            result,
            Err(TemplateError::TemplateNotFound { .. })
        ));
    }
}
