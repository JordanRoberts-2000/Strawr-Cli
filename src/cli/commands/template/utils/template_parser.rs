use crate::template::types::{ParsedTemplateInput, ValidTemplateName, ValidVariantName};

pub fn template_parser(s: &str) -> Result<ParsedTemplateInput, String> {
    if let Some((template, variant)) = s.split_once(':') {
        if template.is_empty() {
            return Err("No valid template name before :".to_string());
        }

        let valid_template: ValidTemplateName = template.parse()?;

        if variant.is_empty() {
            return Ok((valid_template, Some(None)));
        }

        let valid_variant: ValidVariantName = variant.parse()?;

        return Ok((valid_template, Some(Some(valid_variant))));
    }

    let valid_template: ValidTemplateName = s.parse()?;
    Ok((valid_template, None))
}
