use crate::cli::commands::template::TemplateError;

pub fn parse_template(
    template: &String,
    variant: &Option<String>,
) -> Result<(String, String), TemplateError> {
    Ok(("hello".to_string(), "hello".to_string()))
}
