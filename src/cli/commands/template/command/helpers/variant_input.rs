use crate::cli::commands::template::{
    command::{
        service::TemplateService,
        utils::{Template, Variant},
    },
    TemplateError,
};

pub fn resolve_variant(
    service: &TemplateService,
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
            Ok(Some(service.new_variant(&template, &cli_variant)?))
        }
        Some(None) => Ok(Some(service.select_variant(&template, "Select variant:")?)),
        None => match raw_variant {
            Some(v) => Ok(Some(service.new_variant(&template, &v)?)),
            None => Ok(None),
        },
    }
}
