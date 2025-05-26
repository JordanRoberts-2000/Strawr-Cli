use crate::{
    ai::AiError,
    services::{ai::AiServiceError, keyring::KeyringError},
    validation::adaptors::clap::validate,
    CliContext,
};

pub const QUESTION_FORMAT_ERROR: &str = "Invalid question format";
pub const QUESTION_COMPLEX_ERROR: &str = "Question too complex for brief answer";
pub const MAX_TOKENS: u16 = 200;

#[derive(clap::Parser, Debug)]
pub struct AiCommand {
    #[arg(
      help = "Description of what you need a name for",
      long_help = "Describe the function, variable, or concept you need a name for. For example: 'a validation function that ensures input is not empty'",
      value_parser = validate::not_empty
    )]
    pub question: String,
}

#[derive(thiserror::Error, Debug)]
pub enum AiCmdError {
    #[error(transparent)]
    Ai(#[from] AiError),

    #[error(transparent)]
    AiService(#[from] AiServiceError),

    #[error(transparent)]
    Keyring(#[from] KeyringError),

    #[error("{QUESTION_COMPLEX_ERROR}")]
    QuestionTooComplex,

    #[error("{QUESTION_FORMAT_ERROR}")]
    InvalidQuestion,
}

impl AiCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), AiCmdError> {
        let prompt = format!(
      "You are a concise Q&A assistant. Follow these rules strictly:
  
  1. If the input is NOT a valid question (gibberish, commands, requests for code/tasks), respond exactly: '{QUESTION_FORMAT_ERROR}'
  
  2. If the input IS a valid question but requires detailed explanation, respond exactly: '{QUESTION_COMPLEX_ERROR}'
  
  3. If the input IS a valid question that can be answered briefly, provide a clear, factual answer in maximum 20 words.
  
  Input: {}", 
      self.question
  );

        let response = ctx.service.init_ai()?.prompt(&prompt, MAX_TOKENS)?;

        if response == QUESTION_FORMAT_ERROR {
            return Err(AiCmdError::InvalidQuestion);
        } else if response == QUESTION_COMPLEX_ERROR {
            return Err(AiCmdError::QuestionTooComplex);
        }

        println!("{response}");
        Ok(())
    }
}
