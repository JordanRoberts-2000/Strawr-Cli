use crate::template::{
    models::{Template, Variant},
    TemplateView,
};

impl<'a> TemplateView<'a> {
    pub fn template_renamed(&self, template: &Template, new_template: &Template) {
        if self.muted {
            return;
        }
        println!(
            "Successfully renamed template '{}' to '{}' successfully",
            template.id, new_template.id
        )
    }

    pub fn variant_renamed(&self, variant: &Variant, new_variant: &Variant) {
        if self.muted {
            return;
        }
        println!(
            "Successfully renamed variant '{}' to '{}'",
            variant.id, new_variant.id
        )
    }
}
