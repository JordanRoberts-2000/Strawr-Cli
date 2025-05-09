mod command;
mod context;
mod impls {
    mod edit_from_input;
    mod execute;
    mod template_interactive;
    mod variant_interactive;
}

pub use {command::EditSubcommand, context::EditSubcommandContext};
