use crate::{
    services::cli::traits::HasPromptService,
    template::{TemplateError, TemplateView},
};

use colored::*;

impl<'a> TemplateView<'a> {
    pub fn output_not_empty_warning(&self) -> Result<bool, TemplateError> {
        let msg = "The output directory is not empty. Do you still want to inject template files?";
        let input = self.prompt().confirm(msg)?;

        Ok(input)
    }

    pub fn warn_variant_ignored(&self) {
        let msg =
            format!("⚠️  You provided both inline and --variant; only inline was used.").yellow();
        println!("{msg}");
    }
}
