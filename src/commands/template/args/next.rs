use crate::utils::copy_to_client;

pub fn handle_next() {
    if let Err(e) = copy_to_client("assets/templates/next") {
        println!("Error generating Go template: {}", e);
    };
}
