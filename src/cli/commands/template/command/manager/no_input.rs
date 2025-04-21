use crate::{
    cli::commands::template::error::TemplateError,
    utils::{fs::is_dir_empty, input},
};

use super::TemplateManager;

impl TemplateManager {
    pub fn handle_no_input(&self) -> Result<(), TemplateError> {
        log::trace!("No input detected");

        let empty_dir = is_dir_empty(&self.templates_path).map_err(|e| TemplateError::Io {
            context: "could not read template folder".to_string(),
            source: e,
        })?;

        if !empty_dir {
            let template = input::select(&self.templates, "Select template:").prompt()?;
            // check if templates has any variants?
            // self.inject_template_files(template, variant)
            println!("{template}");
        } else {
            return Err(TemplateError::NoTemplates);
        }

        Ok(())
    }
}
