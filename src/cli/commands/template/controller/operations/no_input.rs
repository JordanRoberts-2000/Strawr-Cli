use crate::{
    services::editor_launcher::Editor,
    template::{
        controller::fluent::core::{Start, TemplateFlow},
        TemplateController,
    },
};

impl<'svc> TemplateController<'svc> {
    pub fn handle_no_input<'f>(&'f self, editor: &'f Editor) -> TemplateFlow<'f, Start> {
        TemplateFlow::new(self, editor)
    }
}
