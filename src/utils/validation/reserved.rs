use strum::VariantNames;

pub fn reserved<E: VariantNames>(s: &str) -> Result<String, String> {
    if E::VARIANTS.contains(&s) {
        return Err(format!("`{}` is a reserved value", s));
    }

    Ok(s.to_string())
}
