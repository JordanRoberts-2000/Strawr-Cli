use crate::template::{TemplateContext, TemplateController, TemplateError};

impl TemplateController {
    pub fn handle_stack_flags(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
        let stacks = [
            (ctx.backend, ctx.backend_folder_title),
            (ctx.frontend, ctx.frontend_folder_title),
        ];

        for (template_arg, folder_title) in stacks {
            if let Some((template_part, variant_part)) = template_arg {
                let template = self.service.get_existing_template(template_part)?;

                let target_dir = ctx.output.join(folder_title);
                if !target_dir.exists() {
                    self.service.fs.create_dir_all(&target_dir)?;
                }

                if let Some(v) = variant_part {
                    match v {
                        None => {
                            let variant = self.select_variant(&template)?;
                            self.inject_template_files(&variant.path, &target_dir)?;
                        }
                        Some(v) => {
                            let variant = self.service.get_existing_variant(&template, v)?;
                            self.inject_template_files(&variant.path, &target_dir)?;
                        }
                    };
                    break;
                }

                self.inject_template_files(&template.path, &target_dir)?;
            }
        }

        Ok(())
    }
}
