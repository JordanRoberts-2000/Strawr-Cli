use std::fmt;
use strum_macros::EnumIter;

#[derive(clap::ValueEnum, Clone, Debug, EnumIter)]
pub enum ContextType {
    #[value(aliases = &["v", "var"])]
    Variable,
    #[value(alias = "const")]
    Constant,
    #[value(alias = "fn")]
    Function,
    #[value(aliases = &["struct", "c"])]
    Class,
    #[value(aliases = &["package", "p", "mod", "lib"])]
    Module,
    Hook,
    #[value(alias = "api")]
    Endpoint,
    #[value(alias = "ui")]
    Component,
    #[value(aliases = &["e", "err"])]
    Error,
    #[value(alias = "t")]
    Test,
    #[value(alias = "dt")]
    Table,
    #[value(alias = "df")]
    Field,
}

impl fmt::Display for ContextType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ContextType::Variable => "variable",
            ContextType::Constant => "constant",
            ContextType::Function => "function",
            ContextType::Class => "class",
            ContextType::Module => "module",
            ContextType::Hook => "react-hook",
            ContextType::Endpoint => "endpoint",
            ContextType::Component => "component",
            ContextType::Error => "error",
            ContextType::Test => "test",
            ContextType::Table => "database table",
            ContextType::Field => "database field",
        };
        f.write_str(s)
    }
}
