use crate::{
    template::{TemplateService, TemplateView},
    CliContext,
};

pub struct TemplateController<'ctx> {
    pub service: TemplateService<'ctx>,
    pub view: TemplateView<'ctx>,
}

impl<'ctx> TemplateController<'ctx> {
    pub fn new(ctx: &'ctx CliContext) -> Self {
        Self {
            service: TemplateService::new(&ctx),
            view: TemplateView::new(&ctx),
        }
    }
}
