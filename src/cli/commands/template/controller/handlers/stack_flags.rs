use crate::template::{TemplateContext, TemplateController, TemplateError};

impl TemplateController {
    pub fn handle_stack_flags(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
        let stacks = [
            (ctx.backend, ctx.backend_folder_title),
            (ctx.frontend, ctx.frontend_folder_title),
        ];

        for (maybe_override, folder_title) in stacks {
            if let Some((raw_template, raw_variant)) = maybe_override {
                let template = self.service.get_existing_template(raw_template)?;

                let target_dir = ctx.output.join(folder_title);
                if !target_dir.exists() {
                    self.service.fs.create_dir_all(&target_dir)?;
                }

                if let Some(v) = raw_variant {
                    let variant = self.service.get_existing_variant(&template, v)?;
                    self.inject_template_files(&variant.path, &target_dir)?;
                    break;
                }

                self.inject_template_files(&template.path, &target_dir)?;
            }
        }

        Ok(())
    }
}
