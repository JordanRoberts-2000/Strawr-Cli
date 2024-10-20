use crate::utils::{copy_to_client, run_command};

pub fn handle_react() {
    if let Err(e) = copy_to_client("assets/templates/react") {
        println!("Error generating react template: {}", e);
    };
    if let Err(e) = run_command("npm i") {
        panic!("Error running npm i: {}", e);
    };
}
