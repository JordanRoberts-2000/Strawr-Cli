use crate::state::AppContext;

use super::args::TemplateCommand;

impl TemplateCommand {
    pub fn execute(&self, ctx: &AppContext) {
        if let Some(template) = &self.template {
            println!("template: {}", template);
        }

        if let Some(variant) = &self.variant {
            println!("variant: {}", variant);
        }

        if let Some(backend) = &self.backend {
            println!("backend: {}", backend);
        }

        if let Some(frontend) = &self.frontend {
            println!("frontend: {}", frontend);
        }
    }
}
