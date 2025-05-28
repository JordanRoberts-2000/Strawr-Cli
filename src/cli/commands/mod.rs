// pub mod backup;
pub mod config;
// pub mod grab;
pub mod img;
// pub mod temp;
pub mod ai;
pub mod suggest;
pub mod template;

pub use {
    ai::AiCommand, config::ConfigCommand, img::ImgCommand, suggest::SuggestCommand,
    template::TemplateCommand,
};

pub mod errors {
    pub use super::{
        ai::AiCmdError, config::ConfigCommandError, img::ImgCmdError, suggest::SuggestCmdError,
        template::TemplateError,
    };
}
