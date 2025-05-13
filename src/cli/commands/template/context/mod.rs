mod context;
mod sub_commands {
    pub mod create;
    pub mod delete;
    pub mod edit;
    pub mod rename;
}

pub use context::TemplateContext;

pub use sub_commands::{
    create::CreateSubcommandContext, delete::DeleteSubcommandContext, edit::EditSubcommandContext,
    rename::RenameSubcommandContext,
};
