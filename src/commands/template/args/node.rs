use crate::{
    commands::template::enums::NodeVariants,
    utils::{copy_to_client, run_command},
};

pub fn handle_node(matches: &clap::ArgMatches) {
    let variant = matches
        .get_one::<NodeVariants>("variant")
        .unwrap_or(&NodeVariants::Express);
    match variant {
        NodeVariants::Vanilla => {
            if let Err(e) = copy_to_client("assets/templates/node/vanilla") {
                panic!("Error generating Node template: {}", e);
            };
            if let Err(e) = run_command("npm i") {
                panic!("Error running npm i: {}", e);
            };
        }
        NodeVariants::Express => {
            if let Err(e) = copy_to_client("assets/templates/node/express") {
                panic!("Error generating Node template: {}", e);
            };
            if let Err(e) = run_command("npm i") {
                panic!("Error running npm i: {}", e);
            };
        }
        _ => println!("not a valid node variant"),
    }
}
