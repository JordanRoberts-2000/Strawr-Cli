use super::{ValidTemplateName, ValidVariantName};

pub type ParsedTemplateInput = (ValidTemplateName, Option<Option<ValidVariantName>>);
