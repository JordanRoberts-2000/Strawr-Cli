use crate::{
    template::{constants::TEMPLATES_FOLDER_NAME, TemplateService, TemplateView},
    CliContext,
};

pub struct TemplateController {
    pub service: TemplateService,
    pub view: TemplateView,
}

impl TemplateController {
    pub fn new(ctx: &CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        Self {
            service: TemplateService::new(&templates_path),
            view: TemplateView::new(&ctx.config.mute),
        }
    }
}
