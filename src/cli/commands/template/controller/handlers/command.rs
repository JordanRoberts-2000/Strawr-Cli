use crate::{
    template::{
        controller::enums::VariantArgEmpty, TemplateCommand, TemplateController, TemplateError,
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

        // self.handle_no_input()
        //     .if_no_templates(NoTemplates::PromptCreation)
        //     .select(TemplateSelect::TemplateOrVariant)
        //     .inject_files(&args.output)?;

        // self.handle_no_input()
        //     .no_templates(NoTemplates::Msg(
        //         "Can't delete any templates as no templates currently exist",
        //     ))
        //     .select(TemplateSelect::DefaultOrVariant)
        //     .inject_files(&args.output)?;

        // self.handle_no_input()
        //     .no_templates(NoTemplates::Msg(
        //         "Can't list templates as no templates currently exist",
        //     ))
        //     .list_templates()?;

        Ok(())
    }
}
