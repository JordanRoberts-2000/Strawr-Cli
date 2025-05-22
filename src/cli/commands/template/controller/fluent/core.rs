use std::marker::PhantomData;

use crate::{
    services::editor_launcher::Editor,
    template::{
        controller::enums::TemplateSelect,
        models::{markers::Exists, Template, Variant},
        TemplateController, TemplateError,
    },
};

pub struct Start;
pub struct NoTemplatesHandled;
pub struct Selected;
pub struct Action;

pub trait CanSelect {}

impl CanSelect for Start {}
impl CanSelect for NoTemplatesHandled {}

pub struct TemplateFlow<'c, S> {
    pub _state: PhantomData<S>,
    pub ctrl: &'c TemplateController<'c>,
    pub handled_no_templates: bool,
    pub template: Option<Template<Exists>>,
    pub variant: Option<Variant<Exists>>,
    pub editor: Editor,
}

impl<'c> TemplateFlow<'c, Start> {
    pub fn new(ctrl: &'c TemplateController, editor: &Editor) -> Self {
        Self {
            _state: PhantomData,
            ctrl: &ctrl,
            handled_no_templates: false,
            template: None,
            variant: None,
            editor: editor.clone(),
        }
    }
}
