use crate::{services::cli::traits::HasPromptService, CliContext, CliService};

pub struct TemplateView<'ctx> {
    pub cli_svc: &'ctx CliService,
    pub muted: bool,
}

impl<'ctx> TemplateView<'ctx> {
    pub fn new(ctx: &'ctx CliContext) -> Self {
        Self {
            cli_svc: &ctx.service,
            muted: ctx.config.quiet_mode,
        }
    }

    pub fn msg(&self, msg: &str) {
        println!("{msg}");
    }
}

impl<'ctx> HasPromptService for TemplateView<'ctx> {
    fn cli(&self) -> &CliService {
        self.cli_svc
    }
}
