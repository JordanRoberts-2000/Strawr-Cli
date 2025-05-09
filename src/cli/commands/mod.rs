// pub mod backup;
// pub mod config;
// pub mod grab;
// pub mod img;
// pub mod temp;
pub mod template;

pub use self::template::TemplateCommand;

pub mod errors {
    pub use super::template::TemplateError;
}
