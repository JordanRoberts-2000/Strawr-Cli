pub fn aspect_ratio_parse(s: &str) -> Result<f32, String> {
    let s = s.trim();

    let mut parts = s.split(':');
    let w_str = parts.next().ok_or_else(|| {
        format!(
            "Invalid aspect ratio structure, `{}` needs to be 'WIDTH:HEIGHT'",
            s
        )
    })?;
    let h_str = parts.next().ok_or_else(|| {
        format!(
            "Invalid aspect ratio structure, `{}` needs to be 'WIDTH:HEIGHT'",
            s
        )
    })?;

    if parts.next().is_some() {
        return Err(format!(
            "Invalid aspect ratio `{}`: too many ':' separators",
            s
        ));
    }

    let w = w_str
        .parse::<u8>()
        .map_err(|e| format!("Invalid width `{}` in aspect ratio: {}", w_str, e))?;
    let h = h_str
        .parse::<u8>()
        .map_err(|e| format!("Invalid height `{}` in aspect ratio: {}", h_str, e))?;

    if h == 0 {
        return Err("Aspect-ratio height must be nonzero".to_string());
    }

    if w == 0 {
        return Err("Aspect-ratio width must be nonzero".to_string());
    }

    Ok(w as f32 / h as f32)
}
