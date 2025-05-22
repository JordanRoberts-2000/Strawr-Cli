use crate::{
    template::{
        controller::enums::VariantArgEmpty, TemplateCommand, TemplateController, TemplateError,
    },
    utils, CliContext,
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
                let (template, variant) =
                    self.resolve_template_arg(&template_arg, &VariantArgEmpty::Select)?;

                let target_dir = args.output.join(folder_title);
                if !target_dir.exists() {
                    utils::fs::create_dir_all(&target_dir)?;
                }

                match variant {
                    Some(v) => self.inject_variant_files(&v, &target_dir)?,
                    None => self.inject_template_files(&template, &target_dir)?,
                };
            }
        }

        Ok(())
    }
}
