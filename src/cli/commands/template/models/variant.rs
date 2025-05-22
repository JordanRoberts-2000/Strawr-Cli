use std::{marker::PhantomData, path::PathBuf};

use crate::template::{
    models::Template,
    types::{ValidTemplateName, ValidVariantName},
    TemplateError,
};

use super::markers::{DoesNotExist, Exists, Unchecked};

#[derive(Debug, Clone)]
pub struct VariantCore {
    pub id: ValidVariantName,
    pub path: PathBuf,
    pub template: Template<Exists>,
}

#[derive(Debug, Clone)]
pub struct Variant<S = Unchecked> {
    core: VariantCore,
    _state: PhantomData<S>,
}

impl<S> Variant<S> {
    pub fn new(template: &Template<Exists>, id: &ValidVariantName) -> Self {
        let path = template.path().join(id.as_str());
        Self {
            core: VariantCore {
                id: id.clone(),
                path,
                template: template.clone(),
            },
            _state: PhantomData,
        }
    }

    pub fn id(&self) -> &ValidVariantName {
        &self.core.id
    }

    pub fn path(&self) -> &PathBuf {
        &self.core.path
    }

    pub fn template_id(&self) -> &ValidTemplateName {
        &self.core.template.id()
    }

    pub fn template(&self) -> &Template<Exists> {
        &self.core.template
    }
}

impl Variant<Unchecked> {
    pub fn ensure_exists(&self) -> Result<Variant<Exists>, TemplateError> {
        if !self.path().exists() {
            return Err(TemplateError::VariantNotFound {
                variant: self.id().to_string(),
                template: self.template_id().to_string(),
            });
        }

        Ok(Variant {
            core: self.core.clone(),
            _state: PhantomData,
        })
    }

    pub fn ensure_does_not_exist(&self) -> Result<Variant<DoesNotExist>, TemplateError> {
        if self.path().exists() {
            return Err(TemplateError::VariantAlreadyExists {
                variant: self.id().to_string(),
                template: self.template_id().to_string(),
            });
        }

        Ok(Variant {
            core: self.core.clone(),
            _state: PhantomData,
        })
    }
}

impl From<&Variant<DoesNotExist>> for Variant<Exists> {
    fn from(v: &Variant<DoesNotExist>) -> Self {
        Variant {
            core: v.core.clone(),
            _state: PhantomData,
        }
    }
}

impl From<Variant<DoesNotExist>> for Variant<Exists> {
    fn from(v: Variant<DoesNotExist>) -> Self {
        Variant {
            core: v.core,
            _state: PhantomData,
        }
    }
}

impl From<Variant<Exists>> for Variant<Unchecked> {
    fn from(v: Variant<Exists>) -> Variant<Unchecked> {
        Variant {
            core: v.core,
            _state: PhantomData,
        }
    }
}

impl From<&Variant<Exists>> for Variant<Unchecked> {
    fn from(v: &Variant<Exists>) -> Variant<Unchecked> {
        Variant {
            core: v.core.clone(),
            _state: PhantomData,
        }
    }
}
