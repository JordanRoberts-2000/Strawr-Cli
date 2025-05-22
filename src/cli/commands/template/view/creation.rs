use crate::template::{
    models::{markers::Exists, Template, Variant},
    TemplateView,
};

impl<'a> TemplateView<'a> {
    pub fn template_created(&self, template: &Template<Exists>) {
        if self.muted {
            return;
        }
        println!("Template '{}' created successfully", template.id());
    }

    pub fn variant_created(&self, variant: &Variant<Exists>) {
        if self.muted {
            return;
        }
        println!(
            "Variant '{}' for template '{}' created successfully",
            variant.id(),
            variant.template_id()
        );
    }
}
