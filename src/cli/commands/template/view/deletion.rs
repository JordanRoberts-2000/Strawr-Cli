use crate::template::{
    models::{markers::Exists, Template, Variant},
    TemplateView,
};

impl<'a> TemplateView<'a> {
    pub fn template_deleted(&self, template: &Template<Exists>) {
        if self.muted {
            return;
        }
        println!("Template '{}' deleted successfully", template.id());
    }
    pub fn variant_deleted(&self, variant: &Variant<Exists>) {
        if self.muted {
            return;
        }
        println!(
            "Variant '{}' from template '{}' deleted successfully",
            variant.id(),
            variant.template_id()
        );
    }
}
