use crate::{
    cli::commands::template::error::TemplateError,
    error::io::IoError,
    utils::{
        fs::{is_dir_empty, subfolders},
        input,
    },
};

use super::TemplateManager;

impl<'a> TemplateManager<'a> {
    pub fn handle_no_input(&self) -> Result<(), TemplateError> {
        let empty_dir = is_dir_empty(&self.templates_path)
            .map_err(|e| IoError::ReadDir(e, self.templates_path.clone()))?;

        if !empty_dir {
            let subdirs =
                subfolders(&self.templates_path).map_err(TemplateError::NoExistingTemplate)?;
            let template = input::select(&subdirs, "Select template:").prompt()?;
            // check if templates has any variants?
            // self.inject_template_files(template, variant)
            println!("{template}");
        } else {
            return Err(TemplateError::NoTemplates);
        }

        Ok(())
    }
}
