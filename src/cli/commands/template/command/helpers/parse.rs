use crate::cli::commands::template::TemplateError;

pub fn parse_input(
    template: &String,
    cli_variant: &Option<String>,
) -> Result<(String, Option<String>), TemplateError> {
    if template.contains(':') && cli_variant.is_some() {
        log::warn!(
            "':' detected in template input (`{template}`) but --variant was also provided. Using --variant and ignoring inline variant."
        );
    }

    if let Some(variant) = cli_variant {
        return Ok((template.clone(), Some(variant.clone())));
    }

    if template.contains(':') {
        let parts: Vec<&str> = template.split(':').collect();
        let lang = parts.get(0).copied().unwrap_or("").to_string();
        let variant = parts.get(1).map(|s| s.to_string());
        return Ok((lang, variant));
    }

    Ok((template.clone(), None))
}
