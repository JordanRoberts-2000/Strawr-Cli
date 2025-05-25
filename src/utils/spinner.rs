use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub fn spinner<R, E, F>(msg: &str, op: F) -> Result<R, E>
where
    F: FnOnce() -> Result<R, E>,
{
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));

    // run the operation
    let res = op();

    pb.finish_and_clear();
    res
}
