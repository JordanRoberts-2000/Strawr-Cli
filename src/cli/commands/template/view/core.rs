use crate::{services::prompt::PromptService, CliContext};

pub struct TemplateView<'a> {
    pub prompt: &'a PromptService,
    pub muted: bool,
}

impl<'a> TemplateView<'a> {
    pub fn new(ctx: &'a CliContext) -> Self {
        Self {
            prompt: ctx.service.prompt(),
            muted: ctx.config.quiet,
        }
    }
}
