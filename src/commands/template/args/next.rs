use crate::commands::template::utils::copy_template;

pub fn handle_next() {
    if let Err(e) = copy_template("next") {
        println!("Error generating Go template: {}", e);
    };
}
