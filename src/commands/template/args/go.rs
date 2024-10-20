use crate::utils::copy_to_client;

pub fn handle_go() {
    if let Err(e) = copy_to_client("assets/templates/go") {
        println!("Error generating Go template: {}", e);
    };
}
