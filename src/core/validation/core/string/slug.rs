use crate::validation::ValidationError;

const MAX_SLUG_LENGTH: usize = 50;

pub fn slug(input: impl AsRef<str>) -> Result<String, ValidationError> {
    let input = input.as_ref();
    let slug = slug::slugify(input.trim());

    if slug.is_empty() {
        return Err(ValidationError::EmptySlug(input.to_string()));
    }

    if slug.len() > MAX_SLUG_LENGTH {
        return Err(ValidationError::SlugTooLong {
            max: MAX_SLUG_LENGTH,
            slug,
        });
    }

    Ok(slug)
}
