use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub fn spinner(msg: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    spinner.set_message(msg.to_string());
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner
    // spinner.finish_and_clear();
}
