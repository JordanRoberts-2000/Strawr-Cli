use std::{fmt, str};

use crate::{template::TemplateSubcommand, utils::validation};

#[derive(Debug, Clone)]
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
        let template = validation::slug(s)?;
        validation::reserved::<TemplateSubcommand>(&template)?;

        Ok(Self(template))
    }
}

impl fmt::Display for ValidVariantName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
