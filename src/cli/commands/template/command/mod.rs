mod command;
pub mod sub_commands {
    mod create;
    mod delete;
    mod edit;
    mod rename;

    pub use {
        create::CreateSubcommand, delete::DeleteSubcommand, edit::EditSubcommand,
        rename::RenameSubcommand,
    };
}

pub use command::TemplateCommand;
pub(crate) use command::TemplateSubcommand;
