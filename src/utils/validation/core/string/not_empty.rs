use crate::utils::validation::ValidationError;

pub fn is_empty(input: impl AsRef<str>) -> Result<String, ValidationError> {
    let input = input.as_ref().trim();

    if input.is_empty() {
        return Err(ValidationError::Empty);
    }

    Ok(input.to_owned())
}
