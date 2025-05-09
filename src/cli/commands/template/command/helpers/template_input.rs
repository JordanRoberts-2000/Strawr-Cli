use crate::template::{types::TemplateInput, TemplateContext, TemplateError, TemplateService};

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
