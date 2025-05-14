use crate::{
    services::editor_launcher::Editor,
    template::{models::Template, types::ValidTemplateName, TemplateController, TemplateError},
};

impl TemplateController {
    pub fn create_template(
        &self,
        template: &ValidTemplateName,
        editor: &Editor,
    ) -> Result<Template, TemplateError> {
        let template = self.service.create_template(template)?;
        self.view.template_created(&template);
        self.service
            .launch_editor(&editor, &template.default_path())?;

        Ok(template)
    }
}
