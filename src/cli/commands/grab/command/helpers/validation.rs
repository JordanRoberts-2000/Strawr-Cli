use regex::Regex;

use crate::services::crypto::ENCRYPTION_PREFIX;

pub fn validate_key(s: &str) -> Result<String, String> {
    let re = Regex::new(r"^[a-zA-Z0-9_-]{1,20}$").map_err(|e| e.to_string())?;
    if re.is_match(s) {
        Ok(s.to_owned())
    } else {
        Err(
            "Key must contain only letters, digits, '-' or '_' and be between 1 and 20 characters"
                .into(),
        )
    }
}

pub fn validate_value(s: &str) -> Result<String, String> {
    let max_length = 60;

    if s.len() > max_length {
        return Err(format!("Value can't contain more than {} characters", max_length).into());
    }
    if s.trim().len() == 0 {
        return Err("Value must contain atleast 1 non-space value".into());
    }
    if s.starts_with(ENCRYPTION_PREFIX) {
        return Err(format!("Value can't start with '{}', as this is the internal prefix for detecting encrypted data", ENCRYPTION_PREFIX).into());
    }

    Ok(s.to_owned())
}
