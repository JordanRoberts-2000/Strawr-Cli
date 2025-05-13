use crate::{
    template::{TemplateCommand, TemplateContext, TemplateController, TemplateError},
    CliContext,
};

impl TemplateController {
    pub fn execute(&self, args: &TemplateCommand, ctx: &CliContext) -> Result<(), TemplateError> {
        self.service.init_templates_folder()?;

        if let Some(subcommand) = &args.subcommand {
            return self.handle_subcommands(subcommand, &ctx);
        }

        let ctx = TemplateContext::new(args, ctx);

        // todo: replace with
        // if let Some(input) = &args.template {
        //   return self.handle_template_and_variant(&input, &args.variant).inject_files(&ctx.output);
        //   }?;

        if let Some(input) = &args.template {
            let (template, variant) = self.resolve_template_and_variant(&input, &args.variant)?;
            let path = match variant {
                Some(v) => v.path,
                None => template.path,
            };

            return self.inject_template_files(&path, &ctx.output);
        }

        if args.backend.is_some() || args.frontend.is_some() {
            return self.handle_stack_flags(&ctx);
        }

        // todo: replace with
        // self.when_no_input(&ctx)
        // .select(Selection::TemplateOnly)
        // .on_empty_error()
        // .then_inject(&ctx.output)?;
        self.handle_no_input(&ctx)?;
        Ok(())
    }
}
