use std::time::Instant;

pub fn time_execution<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let start_time = Instant::now();

    let result = f();

    let duration = start_time.elapsed();
    log::info!("⏱️ Completed in {:.2?}", duration);

    result
}
