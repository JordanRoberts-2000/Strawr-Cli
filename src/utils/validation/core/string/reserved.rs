use strum::VariantNames;

use crate::utils::validation::ValidationError;

pub fn reserved<E: VariantNames>(str: impl AsRef<str>) -> Result<String, ValidationError> {
    let s = str.as_ref();
    if E::VARIANTS.contains(&s) {
        return Err(ValidationError::Reserved(s.to_string()));
    }

    Ok(s.to_string())
}
