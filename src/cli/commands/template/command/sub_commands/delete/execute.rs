use crate::{
    cli::commands::template::{service::TemplateService, TemplateError},
    state::AppContext,
};

use super::{context::DeleteSubcommandContext, DeleteSubcommand};

impl DeleteSubcommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), TemplateError> {
        let ctx = DeleteSubcommandContext::new(self, ctx);
        let service = TemplateService::from(&ctx);

        if let Some((raw_template, raw_variant)) = &self.template {
            return self.delete_from_input(&service, raw_template, raw_variant.as_deref());
        }

        if self.variant {
            return self.delete_variant_interactive(&service);
        }

        self.delete_template_interactive(&service)
    }
}
