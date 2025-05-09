mod command;
mod context;
mod impls {
    mod create_from_input;
    mod execute;
    mod template_interactive;
    mod variant_interactive;
}

pub use command::CreateSubcommand;
use context::CreateSubcommandContext;
