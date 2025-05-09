mod command;
mod context;
mod impls {
    mod delete_from_input;
    mod execute;
    mod template_interactive;
    mod variant_interactive;
}

pub use {command::DeleteSubcommand, context::DeleteSubcommandContext};
