use crate::{commands::template::utils::copy_template, utils::run_command};

pub fn handle_react() {
    if let Err(e) = copy_template("react") {
        println!("Error generating react template: {}", e);
    };
    if let Err(e) = run_command("npm i") {
        panic!("Error running npm i: {}", e);
    };
}
