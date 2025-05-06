use crate::cli::commands::template::{
    command::{context::TemplateContext, service::TemplateService, TemplateInput},
    TemplateError,
};

use super::variant_input::resolve_variant;

pub fn handle_template_input(
    ctx: &TemplateContext,
    service: &TemplateService,
    input: &TemplateInput,
) -> Result<(), TemplateError> {
    let (raw_template, raw_variant) = input;
    let template = service.new_template(raw_template)?;

    let variant = resolve_variant(&service, &template, &ctx.variant, &raw_variant)?;
    service.inject(&template, variant.as_ref(), &ctx.output)?;

    Ok(())
}
