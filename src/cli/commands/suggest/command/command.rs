use clap::{Parser, Subcommand};

use crate::{commands::suggest::enums::ContextType, validation::adaptors::clap::validate};

#[derive(Parser, Debug)]
#[command(about = "Generate word alternatives or suggest names from descriptions")]
pub struct SuggestCommand {
    #[command(subcommand)]
    pub subcommand: SuggestSubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SuggestSubCommand {
    #[command(about = "Get alternative words from an existing word")]
    Alts(AltsSubcommand),

    #[command(about = "Generate name suggestions from a description")]
    Name(NameSubcommand),
}

#[derive(Parser, Debug)]
pub struct AltsSubcommand {
    #[arg(help = "The word to find alternatives for (e.g., 'happy', 'is_empty')", value_parser = validate::not_empty)]
    pub name: String,

    #[arg(
      long, short, 
      help = "Description to help generate more relevant alternatives",
      long_help = "Optional context to provide better alternatives. For example: 'validation function' or 'error handling'", 
      value_parser = validate::not_empty
    )]
    pub description: Option<String>,

    #[arg(
      short, long, num_args = 0..=1,
      help = "Type of name to generate alternatives for"
    )]
    pub context: Option<Option<ContextType>>,
}

#[derive(Parser, Debug)]
pub struct NameSubcommand {
    #[arg(
      help = "Description of what you need a name for",
      long_help = "Describe the function, variable, or concept you need a name for. For example: 'a validation function that ensures input is not empty'",
      value_parser = validate::not_empty
    )]
    pub description: String,

    #[arg(
      short, long, num_args = 0..=1,
      help = "Type of name to generate alternatives for"
    )]
    pub context: Option<Option<ContextType>>,
}