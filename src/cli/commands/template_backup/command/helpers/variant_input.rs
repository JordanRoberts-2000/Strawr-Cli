use crate::template::{
    utils::{Template, Variant},
    TemplateError, TemplateManager,
};

pub fn resolve_variant(
    manager: &TemplateManager,
    template: &Template,
    variant: &Option<Option<String>>,
    raw_variant: &Option<String>,
) -> Result<Option<Variant>, TemplateError> {
    match variant {
        Some(Some(cli_variant)) => {
            if raw_variant.is_some() {
                log::warn!(
        "':' detected in template input, but --variant was also provided. Using --variant and ignoring inline variant."
    );
            }
            Ok(Some(manager.new_variant(&template, &cli_variant)?))
        }
        Some(None) => Ok(Some(manager.select_variant(&template, "Select variant:")?)),
        None => match raw_variant {
            Some(v) => Ok(Some(manager.new_variant(&template, &v)?)),
            None => Ok(None),
        },
    }
}
