use crate::{cli::commands::template::TemplateError, state::AppContext};

use super::{
    args::TemplateCommand,
    helpers::{handle_no_input, inject_template_files, parse_template},
    manager::TemplateManager,
};

impl TemplateCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        let mut manager = TemplateManager::new(ctx);

        manager.init_storage()?;
        manager.load_templates()?;

        if self.template.is_none() && self.subcommand.is_none() {
            return handle_no_input();
        }

        if let Some(subcommand) = &self.subcommand {
            return subcommand.execute(ctx);
        }

        if let Some(template) = &self.template {
            let (template, variant) = parse_template(template, &self.variant)?;
            inject_template_files(&template, &variant)?;
        }

        Ok(())
    }
}

// if let Some(template) = &self.template {
//   let (template, variant) = get_variant(template)?;
//   copy_template(".", template, variant);
// }

// if let (Some(backend), Some(frontend)) = (&self.backend, &self.frontend) {
//   let (template, variant) = get_variant(backend)?;
//   // create backend folder, let path =
//   copy_template(path, template, variant);

//   let (template, variant) = get_variant(frontend)?;
//   // create frontend folder, let path =
//   copy_template(path, template, variant);
// }

// if let Some(template) = &self.template {
//   let parts: Vec<&str> = template.split(':').collect();
//   let lang = parts.get(0).copied().unwrap_or("unknown");
//   let mut variant = parts.get(1).copied();

//   if let Some(cli_variant) = &self.variant {
//       if variant.is_some() {
//           log::warn!(
//             "Both inline variant `{}` and --variant `{}` were provided. Using --variant.",
//             variant.unwrap_or(""),
//             cli_variant
//         );
//       }
//       variant = Some(cli_variant); // override inline variant
//   }

//   let variant = variant.unwrap_or("default");
//   println!("Template: {} with variant: {}", lang, variant);
// }
