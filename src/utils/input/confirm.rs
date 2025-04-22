use inquire::Confirm;

pub fn confirm_action(prompt: &str) -> Confirm {
    Confirm::new(prompt).with_default(false)
}
