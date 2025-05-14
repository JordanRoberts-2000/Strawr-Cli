use std::{fmt, str};

use crate::{template::constants::DEFAULT_FOLDER, utils::validation};

#[derive(Debug, Clone)]
pub struct ValidTemplateName(String);
impl ValidTemplateName {
    pub fn new(str: &str) -> Self {
        Self(str.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl str::FromStr for ValidTemplateName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let variant = validation::slug(s)?;

        if variant == DEFAULT_FOLDER {
            return Err(format!("'{DEFAULT_FOLDER}' is a reserved value"));
        }

        Ok(Self(variant))
    }
}

impl fmt::Display for ValidTemplateName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
