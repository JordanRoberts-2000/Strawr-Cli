use crate::{
    template::{TemplateCommand, TemplateController, TemplateError},
    CliContext,
};

impl TemplateController {
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

        // todo - add builder:
        // self.when_no_input(&ctx)
        // .select(Selection::TemplateOnly)
        // .if_no_templates_show("No templates to delete")
        // .then_inject(&ctx.output)?;
        Ok(())
    }
}
