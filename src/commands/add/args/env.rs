use std::{env, io};

use crate::{
    commands::add::utils::{
        add_code_to_main::rust::add_to_main_fn,
        add_import::{node::add_import_to_node, rust::add_import_to_main_rs},
        add_to_gitignore::add_to_gitignore,
        detect_codebase::{detect_codebase, CodeBase},
    },
    utils::{copy_to_client_dir, create_file, run_command},
};

pub fn add_env() {
    if let Err(e) = create_file(".env", "") {
        panic!("Error creating .env file: {}", e);
    };
    if detect_codebase(CodeBase::Node) || detect_codebase(CodeBase::React) {
        if let Err(e) = run_command("npm i dotenv") {
            panic!("Error installing dotenv: {}", e);
        };
        if detect_codebase(CodeBase::Node) {
            if let Err(e) = add_import_to_node("import \"./utils/initialize_env\";") {
                panic!("Error adding dotenv to node: {}", e);
            };
            if let Err(e) = get_initialize_env_file() {
                panic!("Error adding dotenv to node: {}", e);
            };
        }
    }
    if detect_codebase(CodeBase::Rust) {
        if let Err(e) = run_command("cargo add dotenv") {
            panic!("Error installing dotenv: {}", e);
        };
        if let Err(e) = add_import_to_main_rs("use dotenv::dotenv;") {
            panic!("Error adding dotenv: {}", e);
        };
        let initialize_env =
            "\t\tif let Err(err) = dotenv() {\n\t\t\t\tpanic!(\"Error loading .env file: {}\", err);\n\t\t}";
        if let Err(e) = add_to_main_fn(initialize_env) {
            panic!("Error adding dotenv: {}", e);
        };
    }
    if let Err(e) = add_to_gitignore(".env") {
        panic!("Error handling git-ignore: {}", e);
    };
}

fn get_initialize_env_file() -> io::Result<()> {
    let current_dir = env::current_dir().expect("could not get current dir");
    let src_dir = current_dir.join("src");
    if src_dir.exists() {
        copy_to_client_dir("assets/add/initialize_env.ts", "src/utils")?
    } else {
        copy_to_client_dir("assets/add/initialize_env.ts", "utils")?;
    }
    Ok(())
}
