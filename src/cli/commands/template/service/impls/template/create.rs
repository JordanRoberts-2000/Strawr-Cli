use crate::{
    template::{
        models::{
            markers::{DoesNotExist, Exists},
            Template,
        },
        service::TemplateService,
        TemplateError,
    },
    utils,
};

impl<'svc> TemplateService<'svc> {
    pub fn create_template(
        &self,
        template: &Template<DoesNotExist>,
    ) -> Result<Template<Exists>, TemplateError> {
        utils::fs::create_dir_all(&template.default_path())?;

        Ok(Template::<Exists>::from(template))
    }
}
