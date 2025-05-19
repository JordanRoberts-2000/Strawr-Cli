use crate::template::{
    models::{Template, Variant},
    TemplateView,
};

impl<'a> TemplateView<'a> {
    pub fn template_created(&self, template: &Template) {
        if self.muted {
            return;
        }
        println!("Template '{}' created successfully", template.id);
    }

    pub fn variant_created(&self, variant: &Variant) {
        if self.muted {
            return;
        }
        println!(
            "Variant '{}' for template '{}' created successfully",
            variant.id,
            variant.get_template_id()
        );
    }
}
