use std::fmt::Display;

use crate::services::prompt::{PromptService, PromptServiceError};

impl PromptService {
    pub fn multi_select<T: Display + Clone>(
        &self,
        options: &[T],
        msg: &str,
    ) -> Result<Vec<T>, PromptServiceError> {
        if options.is_empty() {
            return Err(PromptServiceError::EmptyOptions);
        }

        let str_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let selected_strs = self.repo.multi_select(&str_options, &[], msg)?;

        let mut result = Vec::with_capacity(selected_strs.len());
        for sel in selected_strs {
            let idx = str_options
                .iter()
                .position(|s| s == &sel)
                .ok_or_else(|| PromptServiceError::InvalidResponse(sel.clone()))?;
            result.push(options[idx].clone());
        }

        Ok(result)
    }

    pub fn multi_select_with_defaults<T: Display + Clone>(
        &self,
        options: &[T],
        defaults: &[usize],
        msg: &str,
    ) -> Result<Vec<T>, PromptServiceError> {
        if options.is_empty() {
            return Err(PromptServiceError::EmptyOptions);
        }

        // ensure every default is in-bounds
        let len = options.len();
        for &d in defaults {
            if d >= len {
                return Err(PromptServiceError::InvalidDefaultIndex(d, len));
            }
        }

        let str_options: Vec<String> = options.iter().map(|opt| opt.to_string()).collect();

        let selected_strs = self.repo.multi_select(&str_options, defaults, msg)?;

        let mut result = Vec::with_capacity(selected_strs.len());
        for sel in selected_strs {
            let idx = str_options
                .iter()
                .position(|s| s == &sel)
                .ok_or_else(|| PromptServiceError::InvalidResponse(sel.clone()))?;
            result.push(options[idx].clone());
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;
    use crate::services::prompt::mock::{MockInput, MockInputRepo};

    #[test]
    fn multi_select_returns_correct_string_vec() {
        let options = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let selected = vec!["b".to_string(), "c".to_string()];
        // queue up exactly what the repo should return
        let repo = MockInputRepo::from(vec![MockInput::Checklist(selected.clone())]);

        let mut service = PromptService::new();
        service.set_repo(repo);

        let result = service.multi_select(&options, "Pick some:").unwrap();
        assert_eq!(result, selected);
    }

    #[derive(Debug, PartialEq, Clone)]
    enum Options {
        One,
        Two,
        Three,
    }

    impl fmt::Display for Options {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = match self {
                Options::One => "One",
                Options::Two => "Two",
                Options::Three => "Three",
            };
            write!(f, "{s}")
        }
    }

    #[test]
    fn multi_select_returns_correct_various_types() {
        let opts_enum = vec![Options::One, Options::Two, Options::Three];
        let opts_num = vec![10, 20, 30];
        let opts_bool = vec![true, false];
        let opts_str = vec!["foo", "bar", "baz"];

        let inputs = vec![
            MockInput::Checklist(vec!["One".into()]),
            MockInput::Checklist(vec!["20".into()]),
            MockInput::Checklist(vec!["false".into()]),
            MockInput::Checklist(vec!["bar".into()]),
        ];
        let repo = MockInputRepo::from(inputs);

        let mut service = PromptService::new();
        service.set_repo(repo);

        let e = service.multi_select(&opts_enum, "Pick:").unwrap();
        assert_eq!(e, vec![Options::One]);

        let n = service.multi_select(&opts_num, "Pick:").unwrap();
        assert_eq!(n, vec![20]);

        let b = service.multi_select(&opts_bool, "Pick:").unwrap();
        assert_eq!(b, vec![false]);

        let s = service.multi_select(&opts_str, "Pick:").unwrap();
        assert_eq!(s, vec!["bar"]);
    }

    #[test]
    fn multi_select_with_defaults_ignores_defaults_in_service() {
        // service just passes defaults through; repo mock doesn't inspect them.
        let options = vec!["x".to_string(), "y".to_string()];
        let selected = vec!["y".to_string()];
        let repo = MockInputRepo::from(vec![MockInput::Checklist(selected.clone())]);

        let mut service = PromptService::new();
        service.set_repo(repo);

        let result = service
            .multi_select_with_defaults(&options, &[1], "Pick:")
            .unwrap();
        assert_eq!(result, selected);
    }

    #[test]
    fn multi_select_errors_on_empty_options() {
        let repo = MockInputRepo::new();

        let mut service = PromptService::new();
        service.set_repo(repo);

        let empty: Vec<String> = vec![];
        let err = service.multi_select(&empty, "Pick?").unwrap_err();
        assert!(matches!(err, PromptServiceError::EmptyOptions));
    }

    #[test]
    fn multi_select_with_defaults_errors_on_empty_options() {
        let repo = MockInputRepo::new();

        let mut service = PromptService::new();
        service.set_repo(repo);

        let empty: Vec<String> = vec![];
        let empty_defaults: Vec<usize> = vec![];
        let err = service
            .multi_select_with_defaults(&empty, &empty_defaults, "Pick?")
            .unwrap_err();
        assert!(matches!(err, PromptServiceError::EmptyOptions));
    }

    #[test]
    fn multi_select_with_defaults_errors_on_bad_default() {
        let options = vec!["a".to_string(), "b".to_string()];
        let repo = MockInputRepo::new();

        let mut service = PromptService::new();
        service.set_repo(repo);

        // default index 5 is out of range for len=2
        let err = service
            .multi_select_with_defaults(&options, &[5], "Pick?")
            .unwrap_err();
        assert!(matches!(err, PromptServiceError::InvalidDefaultIndex(5, 2)));
    }
}
