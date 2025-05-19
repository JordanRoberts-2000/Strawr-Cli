use crate::services::prompt::{
    mock::{MockInput, MockInputCall, MockInputRepo},
    traits::MultiSelectPrompt,
    user::UserInputError,
};

impl MultiSelectPrompt for MockInputRepo {
    fn multi_select(
        &self,
        options: &[String],
        defaults: &[usize],
        msg: &str,
    ) -> Result<Vec<String>, UserInputError> {
        self.history.borrow_mut().push(MockInputCall::Checklist {
            msg: msg.to_string(),
            options: options.to_vec(),
            defaults: defaults.to_vec(),
        });

        let input = self
            .inputs
            .borrow_mut()
            .pop_front()
            .expect(format!("Ran out of test inputs for '{msg}'").as_str());

        match input {
            MockInput::Checklist(v) => Ok(v),
            _ => panic!("Expected Confirm input"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_select_returns_values_and_records_history() {
        let options = vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ];
        let defaults = vec![0, 2];

        // Prepare two canned checklist responses:
        let inputs = vec![
            MockInput::Checklist(vec!["banana".into(), "cherry".into()]),
            MockInput::Checklist(vec!["apple".into()]),
        ];
        let repo = MockInputRepo::from(inputs);
        assert!(repo.history.borrow().is_empty());

        // First call
        let got1 = repo
            .multi_select(&options, &defaults, "Pick fruits:")
            .unwrap();
        assert_eq!(got1, vec!["banana".to_string(), "cherry".to_string()]);

        // Second call
        let got2 = repo
            .multi_select(&options, &defaults, "Pick fruits:")
            .unwrap();
        assert_eq!(got2, vec!["apple".to_string()]);

        // History was recorded for both calls
        let hist = repo.history.borrow();
        assert_eq!(hist.len(), 2);
        assert_eq!(
            hist[0],
            MockInputCall::Checklist {
                msg: "Pick fruits:".to_string(),
                options: options.clone(),
                defaults: defaults.clone(),
            }
        );
        assert_eq!(
            hist[1],
            MockInputCall::Checklist {
                msg: "Pick fruits:".to_string(),
                options: options.clone(),
                defaults: defaults.clone(),
            }
        );
    }

    #[test]
    #[should_panic(expected = "Ran out of test inputs for 'Foo?'")]
    fn multi_select_panics_if_no_inputs_left() {
        let options = vec!["x".into(), "y".into()];
        let defaults = vec![1];
        let repo = MockInputRepo::new();
        let _ = repo.multi_select(&options, &defaults, "Foo?");
    }

    #[test]
    #[should_panic(expected = "Expected Confirm input")]
    fn multi_select_panics_on_wrong_variant() {
        let options = vec!["x".into(), "y".into()];
        let defaults = vec![];
        let repo = MockInputRepo::from(vec![MockInput::Confirm(true)]);
        let _ = repo.multi_select(&options, &defaults, "Bar?");
    }
}
