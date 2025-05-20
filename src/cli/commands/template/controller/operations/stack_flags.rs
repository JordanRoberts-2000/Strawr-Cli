use crate::{
    error::IoError,
    template::{TemplateCommand, TemplateController, TemplateError},
    CliContext,
};

impl<'a> TemplateController<'a> {
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
                    std::fs::create_dir_all(&target_dir)
                        .map_err(|e| IoError::CreateDir(e, target_dir.to_path_buf()))?;
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
