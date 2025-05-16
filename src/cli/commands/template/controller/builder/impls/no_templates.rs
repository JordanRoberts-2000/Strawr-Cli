impl<'c> NoInputBuilder<'c> {
    pub fn if_no_templates_show_msg(&mut self, msg: &str) -> &Self {
        self.controller.view.no_templates(msg);
        self.has_no_templates = true;
        self
    }

    pub fn if_no_templates_prompt_creating_one(
        &mut self,
        editor: &Editor,
    ) -> Result<&Self, TemplateError> {
        if self.controller.view.no_templates_prompt_create_one()? {
            let template = self.controller.prompt_template_name()?;
            self.controller.create_template(&template, editor)?;
        }

        self.has_no_templates = true;
        Ok(self)
    }
}
