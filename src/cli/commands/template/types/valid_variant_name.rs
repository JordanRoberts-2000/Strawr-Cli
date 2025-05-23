use std::{fmt, str};

use crate::{template::constants::DEFAULT_FOLDER, utils::validation::adaptors::clap::validate};

#[derive(Debug, Clone, PartialEq)]
pub struct ValidVariantName(String);

impl ValidVariantName {
    pub fn new(str: &str) -> Self {
        Self(str.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl str::FromStr for ValidVariantName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variant = validate::slug(s)?;

        if variant == DEFAULT_FOLDER {
            return Err(format!("'{DEFAULT_FOLDER}' is a reserved value"));
        }

        Ok(Self(variant))
    }
}

impl fmt::Display for ValidVariantName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
