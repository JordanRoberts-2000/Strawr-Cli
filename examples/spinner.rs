use std::{thread, time::Duration};

use strawr::utils::spinner;

fn main() {
    spinner("Working for 3 seconds…", || -> Result<(), ()> {
        thread::sleep(Duration::from_secs(3));
        Ok(())
    })
    .unwrap();

    println!("Done!");
}
