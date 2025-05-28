use super::helpers::validation::*;

#[derive(clap::Parser, Debug)]
pub struct GrabCommand {
    #[arg(value_parser = validate_key)]
    pub key: Option<String>,

    #[arg(short, long, action = clap::ArgAction::SetTrue, conflicts_with = "value")]
    pub delete: bool,

    #[arg(short, long, default_missing_value = "true", num_args = 0..=1)]
    pub encrypt: Option<bool>,

    #[arg(short, long, value_parser = validate_value, conflicts_with = "delete")]
    pub value: Option<String>,
}
