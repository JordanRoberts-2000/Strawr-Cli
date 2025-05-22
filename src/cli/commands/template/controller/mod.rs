mod core;

mod handlers {
    mod command;
    mod sub_commands {
        mod create;
        mod delete;
        mod edit;
        mod rename;
    }
}

mod operations {
    mod no_input;
    mod resolve_template;
    mod resolve_template_arg;
    mod resolve_variant_arg;
    mod stack_flags;
    mod subcommands;
}

mod actions;
pub(crate) mod enums;
pub mod fluent;
pub mod resolver;

pub(crate) use core::TemplateController;
