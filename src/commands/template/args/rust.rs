use crate::utils::copy_to_client;

pub fn handle_rust() {
    if let Err(e) = copy_to_client("assets/templates/rust") {
        println!("Error generating Rust template: {}", e);
    };
}
