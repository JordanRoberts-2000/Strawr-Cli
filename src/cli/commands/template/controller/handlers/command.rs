use crate::{
    template::{TemplateCommand, TemplateController, TemplateError},
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_command(
        &self,
        args: &TemplateCommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        self.service.init_templates_folder()?;

        if let Some(subcommand) = &args.subcommand {
            return self.handle_subcommands(subcommand, &ctx);
        }

        if let Some(input) = &args.template {
            return self
                .resolve_template(&input, &args.variant)?
                .inject_files(&args.output);
        }

        if args.backend.is_some() || args.frontend.is_some() {
            return self.handle_stack_flags(&args, &ctx);
        }

        Ok(())
    }
}
