use std::time::Instant;

pub fn time_execution<F, T, E>(f: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
{
    let start = Instant::now();
    let result = f();

    if result.is_ok() {
        let duration = start.elapsed();
        log::info!("⏱️ Completed in {:.2?}", duration);
    }

    result
}
