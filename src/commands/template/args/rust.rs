use crate::commands::template::utils::copy_template;

pub fn handle_rust() {
    if let Err(e) = copy_template("rust") {
        println!("Error generating Rust template: {}", e);
    };
}
