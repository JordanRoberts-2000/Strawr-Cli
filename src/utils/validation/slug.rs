const MAX_SLUG_LENGTH: usize = 50;

pub fn slug(input: &str) -> Result<String, String> {
    let slug = slug::slugify(input.trim());

    if slug.is_empty() {
        return Err("Slug cannot be empty.".to_string());
    }

    if slug.len() > MAX_SLUG_LENGTH {
        return Err(format!(
            "Slug must be less than {MAX_SLUG_LENGTH} characters (was {}).",
            slug.len()
        ));
    }

    Ok(slug)
}
