use std::fmt::Display;

use inquire::{InquireError, Select};

use crate::utils::input::UserInput;

use super::{build_render_config, InputError};

pub trait SelectWithoutFilterInput {
    fn select_without_filter<T>(&self, options: &Vec<T>, msg: &str) -> Result<String, InputError>
    where
        T: Display + Clone;
}

impl SelectWithoutFilterInput for UserInput {
    fn select_without_filter<T>(&self, options: &Vec<T>, msg: &str) -> Result<String, InputError>
    where
        T: Display + Clone,
    {
        let prompt = Select::new(msg, options.clone())
            .without_help_message()
            .without_filtering()
            .with_page_size(4)
            .with_render_config(build_render_config())
            .prompt();

        match prompt {
            Ok(selected) => Ok(selected.to_string()),
            Err(InquireError::OperationInterrupted | InquireError::OperationCanceled) => {
                Err(InputError::Canceled)
            }
            Err(err) => Err(InputError::PromptError(err)),
        }
    }
}
