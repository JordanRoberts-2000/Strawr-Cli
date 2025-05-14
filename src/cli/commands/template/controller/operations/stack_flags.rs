use crate::{
    template::{TemplateCommand, TemplateController, TemplateError},
    CliContext,
};

impl TemplateController {
    pub fn handle_stack_flags(
        &self,
        args: &TemplateCommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        let stacks = [
            (&args.backend, &ctx.config.template.backend_folder_title),
            (&args.frontend, &ctx.config.template.frontend_folder_title),
        ];

        for (template_arg, folder_title) in stacks {
            if let Some(template_arg) = template_arg {
                let (template, variant) = self.resolve_template_arg(&template_arg)?;

                let target_dir = args.output.join(folder_title);
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
