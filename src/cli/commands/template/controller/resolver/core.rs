use crate::template::{
    models::{Template, Variant},
    TemplateController,
};

pub struct TemplateResolver<'c> {
    pub(super) controller: &'c TemplateController<'c>,
    pub(super) template: Template,
    pub(super) variant: Option<Variant>,
}

impl<'c> TemplateResolver<'c> {
    pub fn new(
        controller: &'c TemplateController,
        template: Template,
        variant: Option<Variant>,
    ) -> Self {
        TemplateResolver {
            controller,
            template,
            variant,
        }
    }
}
