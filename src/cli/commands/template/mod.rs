pub(crate) mod constants {
    pub const DEFAULT_FOLDER: &str = "default";
    pub const TEMPLATES_FOLDER_NAME: &str = "templates";
}

pub(crate) mod types {
    pub type TemplateInput = (String, Option<String>);
}

mod command {
    pub mod command;
    pub(crate) mod context;
    mod helpers {
        pub(super) mod template_input;
        pub(super) mod templates_init;
        mod variant_input;
    }
    mod impls {
        mod execute;
        mod stack_flags;
        mod no_input {
            mod choose_and_apply;
            mod create_initial;
            mod no_input;
        }
    }
    pub mod sub_commands;
}
mod config;
mod error;
mod manager;
pub(crate) mod utils;

pub use self::{command::command::TemplateCommand, command::sub_commands, error::TemplateError};
pub(crate) use self::{
    command::context::TemplateContext, command::sub_commands::TemplateSubcommand,
    config::TemplateConfig, manager::TemplateManager,
};
