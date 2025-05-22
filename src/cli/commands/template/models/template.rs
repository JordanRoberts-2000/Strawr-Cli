use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::template::{constants::DEFAULT_FOLDER, types::ValidTemplateName, TemplateError};

use super::markers::{DoesNotExist, Exists, Unchecked};

#[derive(Debug, Clone)]
pub struct TemplateCore {
    pub id: ValidTemplateName,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Template<S = Unchecked> {
    core: TemplateCore,
    _state: PhantomData<S>,
}

impl<S> Template<S> {
    pub fn new(id: &ValidTemplateName, templates_path: &Path) -> Self {
        let path = templates_path.join(id.as_str());
        Self {
            core: TemplateCore {
                id: id.clone(),
                path,
            },
            _state: PhantomData,
        }
    }

    pub fn id(&self) -> &ValidTemplateName {
        &self.core.id
    }

    pub fn path(&self) -> &PathBuf {
        &self.core.path
    }

    pub fn default_path(&self) -> PathBuf {
        self.core.path.join(DEFAULT_FOLDER)
    }
}

impl Template<Unchecked> {
    pub fn ensure_exists(&self) -> Result<Template<Exists>, TemplateError> {
        if !self.path().exists() {
            return Err(TemplateError::TemplateNotFound(self.id().to_string()));
        }

        Ok(Template {
            core: self.core.clone(),
            _state: PhantomData,
        })
    }

    pub fn ensure_does_not_exist(&self) -> Result<Template<DoesNotExist>, TemplateError> {
        if self.path().exists() {
            return Err(TemplateError::TemplateAlreadyExists(self.id().to_string()));
        }

        Ok(Template {
            core: self.core.clone(),
            _state: PhantomData,
        })
    }
}

impl From<&Template<DoesNotExist>> for Template<Exists> {
    fn from(t: &Template<DoesNotExist>) -> Self {
        Template {
            core: t.core.clone(),
            _state: PhantomData,
        }
    }
}

impl From<Template<DoesNotExist>> for Template<Exists> {
    fn from(t: Template<DoesNotExist>) -> Self {
        Template {
            core: t.core,
            _state: PhantomData,
        }
    }
}

impl From<Template<Exists>> for Template<Unchecked> {
    fn from(v: Template<Exists>) -> Template<Unchecked> {
        Template {
            core: v.core,
            _state: PhantomData,
        }
    }
}

impl From<&Template<Exists>> for Template<Unchecked> {
    fn from(v: &Template<Exists>) -> Template<Unchecked> {
        Template {
            core: v.core.clone(),
            _state: PhantomData,
        }
    }
}
