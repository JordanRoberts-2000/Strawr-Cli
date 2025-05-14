use crate::template::{TemplateContext, TemplateController, TemplateError};

impl TemplateController {
    pub fn handle_stack_flags(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
        let stacks = [
            (ctx.backend, ctx.backend_folder_title),
            (ctx.frontend, ctx.frontend_folder_title),
        ];

        for (template_arg, folder_title) in stacks {
            if let Some(template_arg) = template_arg {
                let (template, variant) = self.resolve_template_arg(&template_arg)?;

                let target_dir = ctx.output.join(folder_title);
                if !target_dir.exists() {
                    self.service.fs.create_dir_all(&target_dir)?;
                }

                match variant {
                    Some(v) => self.inject_template_files(&v.path, &target_dir)?,
                    None => self.inject_template_files(&template.path, &target_dir)?,
                };
            }
        }

        Ok(())
    }
}
