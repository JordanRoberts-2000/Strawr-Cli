use std::fmt;

pub enum VariantArgEmpty {
    Select,
    PromptText,
}

pub enum TemplateSelect {
    TemplateOrVariant,
    DefaultOrVariant,
}

pub enum NoTemplates {
    PromptCreation,
    Msg(String),
}

#[derive(Debug, Clone, Copy)]
pub enum TemplateOrVariant {
    Template,
    Variant,
}

impl fmt::Display for TemplateOrVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateOrVariant::Template => write!(f, "Template"),
            TemplateOrVariant::Variant => write!(f, "Variant"),
        }
    }
}
