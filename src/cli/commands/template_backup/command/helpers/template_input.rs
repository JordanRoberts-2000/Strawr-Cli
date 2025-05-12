use crate::template::{types::TemplateInput, TemplateContext, TemplateError, TemplateManager};

use super::variant_input::resolve_variant;

pub fn handle_template_input(
    ctx: &TemplateContext,
    manager: &TemplateManager,
    input: &TemplateInput,
) -> Result<(), TemplateError> {
    let (raw_template, raw_variant) = input;
    let template = manager.new_template(raw_template)?;

    let variant = resolve_variant(&manager, &template, &ctx.variant, &raw_variant)?;
    manager.inject(&template, variant.as_ref(), &ctx.output)?;

    Ok(())
}
