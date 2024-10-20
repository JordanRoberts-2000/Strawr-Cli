use crate::commands::template::utils::copy_template;

pub fn handle_go() {
    if let Err(e) = copy_template("go") {
        println!("Error generating Go template: {}", e);
    };
}
