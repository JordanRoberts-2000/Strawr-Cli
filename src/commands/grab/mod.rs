use validation::{validate_key, validate_value};

pub mod config;
pub mod handle_command;
pub mod service;
pub mod validation;

#[derive(clap::Parser, Debug)]
pub struct GrabCommand {
    #[arg(required_unless_present = "list", value_parser = validate_key)]
    pub key: Option<String>,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub delete: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub list: bool,

    #[arg(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub encrypt: Option<bool>,

    #[arg(short, long, value_parser = validate_value)]
    pub value: Option<String>,
}
