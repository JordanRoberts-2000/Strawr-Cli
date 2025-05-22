use crate::{
    template::{
        controller::enums::{NoTemplates, TemplateSelect, VariantArgEmpty},
        TemplateCommand, TemplateController, TemplateError,
    },
    CliContext,
};

impl<'a> TemplateController<'a> {
    pub fn handle_command(
        &self,
        args: &TemplateCommand,
        ctx: &CliContext,
    ) -> Result<(), TemplateError> {
        self.service.init_templates_folder()?;
        let editor = args.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        if let Some(subcommand) = &args.subcommand {
            return self.handle_subcommands(subcommand, &ctx);
        }

        if let Some(input) = &args.template {
            return self
                .resolve_template(&input, &args.variant, VariantArgEmpty::Select)?
                .inject_files(&args.output);
        }

        if args.backend.is_some() || args.frontend.is_some() {
            return self.handle_stack_flags(&args, &ctx);
        }

        self.handle_no_input(&editor)
            .if_no_templates(NoTemplates::PromptCreation)?
            .select(TemplateSelect::DefaultOrVariant)?
            .inject_files(&args.output)?;

        Ok(())
    }
}
