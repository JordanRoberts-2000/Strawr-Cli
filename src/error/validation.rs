use std::borrow::Cow;
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn format_validation_errors(errors: &ValidationErrors) -> String {
    let mut messages = Vec::new();

    for (field, field_errors) in errors.errors() {
        match field_errors {
            ValidationErrorsKind::Field(vec_errors) => {
                for error in vec_errors {
                    let error_msg: Cow<'_, str> = if let Some(message) = &error.message {
                        message.clone()
                    } else {
                        Cow::Owned(format!(
                            "Validation error: {} {}",
                            error.code,
                            serde_json::to_string(&error.params).unwrap_or_default()
                        ))
                    };

                    messages.push(format!("{}: {}", field, error_msg));
                }
            }
            ValidationErrorsKind::Struct(nested_errors) => {
                let nested_messages = format_validation_errors(nested_errors)
                    .lines()
                    .map(|line| format!("{}.{}", field, line))
                    .collect::<Vec<_>>()
                    .join("\n");

                if !nested_messages.is_empty() {
                    messages.push(nested_messages);
                }
            }
            ValidationErrorsKind::List(list_errors) => {
                for (index, error) in list_errors {
                    let nested_messages = format_validation_errors(error)
                        .lines()
                        .map(|line| format!("{}[{}].{}", field, index, line))
                        .collect::<Vec<_>>()
                        .join("\n");

                    if !nested_messages.is_empty() {
                        messages.push(nested_messages);
                    }
                }
            }
        }
    }

    messages.join("\n")
}
