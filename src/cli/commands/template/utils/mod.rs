use crate::template::types::TemplateInput;

pub fn parse_template(s: &str) -> Result<TemplateInput, String> {
    if s.contains(':') {
        let parts: Vec<&str> = s.split(':').collect();
        let template = parts.get(0).copied().unwrap_or("").to_string();
        let variant = parts.get(1).map(|s| s.to_string());
        return Ok((template, variant));
    }

    Ok((s.to_string(), None))
}
