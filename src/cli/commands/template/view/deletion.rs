use crate::template::{
    models::{Template, Variant},
    TemplateView,
};

impl<'a> TemplateView<'a> {
    pub fn template_deleted(&self, template: &Template) {
        if self.muted {
            return;
        }
        println!("Template '{}' deleted successfully", template.id);
    }
    pub fn variant_deleted(&self, variant: &Variant) {
        if self.muted {
            return;
        }
        println!(
            "Variant '{}' from template '{}' deleted successfully",
            variant.id,
            variant.get_template_id()
        );
    }
}
