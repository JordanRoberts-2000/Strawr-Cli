use std::fmt::Display;

use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn search<T: Display + Clone>(
        &self,
        options: &[T],
        msg: &str,
    ) -> Result<T, PromptServiceError> {
        if options.is_empty() {
            return Err(PromptServiceError::EmptyOptions);
        }

        let str_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let input = self.repo.search(&str_options, msg)?;

        let idx = str_options
            .iter()
            .position(|s| s == &input)
            .ok_or_else(|| PromptServiceError::InvalidResponse(input.clone()))?;

        Ok(options[idx].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::prompt::mock::{MockInput, MockInputRepo};
    use std::fmt;

    #[test]
    fn search_returns_correct_string() {
        let options = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let repo = MockInputRepo::from(vec![MockInput::Search("b".into())]);

        let mut service = PromptService::new();
        service.set_repo(repo);

        let input = service.search(&options, "Pick one:").unwrap();
        assert_eq!(input, "b".to_string());
    }

    #[derive(PartialEq, Clone, Debug)]
    enum Options {
        OptionOne,
        OptionTwo,
        OptionThree,
    }

    impl fmt::Display for Options {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Options::OptionOne => write!(f, "OptionOne"),
                Options::OptionTwo => write!(f, "OptionTwo"),
                Options::OptionThree => write!(f, "OptionThree"),
            }
        }
    }

    #[test]
    fn search_returns_correct_display_item() {
        let options_enum = vec![Options::OptionOne, Options::OptionTwo, Options::OptionThree];
        let options_num = vec![1, 2, 3];
        let options_bool = vec![true, false];
        let options_str = vec!["hello", "hola"];

        let inputs = vec![
            MockInput::Search(Options::OptionTwo.to_string()),
            MockInput::Search(2.to_string()),
            MockInput::Search(true.to_string()),
            MockInput::Search("hola".to_string()),
        ];
        let repo = MockInputRepo::from(inputs);

        let mut service = PromptService::new();
        service.set_repo(repo);

        let input_enum = service.search(&options_enum, "Pick one:").unwrap();
        assert_eq!(input_enum, Options::OptionTwo);

        let input_num = service.search(&options_num, "Pick one:").unwrap();
        assert_eq!(input_num, 2);

        let input_bool = service.search(&options_bool, "Pick one:").unwrap();
        assert_eq!(input_bool, true);

        let input_str = service.search(&options_str, "Pick one:").unwrap();
        assert_eq!(input_str, "hola");
    }

    #[test]
    fn search_errors_on_empty_options() {
        let repo = MockInputRepo::new();

        let mut service = PromptService::new();
        service.set_repo(repo);

        let empty: Vec<String> = vec![];
        let err = service.search(&empty, "Pick?").unwrap_err();
        assert!(matches!(err, PromptServiceError::EmptyOptions));
    }
}
