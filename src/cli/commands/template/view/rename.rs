use crate::template::{
    models::{
        markers::{DoesNotExist, Exists},
        Template, Variant,
    },
    TemplateView,
};

impl<'a> TemplateView<'a> {
    pub fn template_renamed(
        &self,
        template: &Template<Exists>,
        new_template: &Template<DoesNotExist>,
    ) {
        if self.muted {
            return;
        }
        println!(
            "Successfully renamed template '{}' to '{}' successfully",
            template.id(),
            new_template.id()
        )
    }

    pub fn variant_renamed(&self, variant: &Variant<Exists>, new_variant: &Variant<DoesNotExist>) {
        if self.muted {
            return;
        }
        println!(
            "Successfully renamed variant '{}' to '{}'",
            variant.id(),
            new_variant.id()
        )
    }
}
