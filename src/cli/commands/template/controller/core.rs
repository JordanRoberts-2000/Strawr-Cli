use crate::{
    template::{TemplateService, TemplateView},
    CliContext,
};

pub struct TemplateController<'a> {
    pub service: TemplateService,
    pub view: TemplateView<'a>,
}

impl<'a> TemplateController<'a> {
    pub fn new(ctx: &'a CliContext) -> Self {
        Self {
            service: TemplateService::new(&ctx),
            view: TemplateView::new(&ctx),
        }
    }
}
