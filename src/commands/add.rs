use std::io::Write;
use std::{env, fs::File, io, path::Path};

use clap::{Arg, Command, ValueEnum};

#[derive(ValueEnum, Clone)]
enum AddVariant {
    Env,
    Jwt,
    Session,
    NextAuth,
    PassportJwt,
    PassportSession,
    Firebase,
}

pub fn add_command() -> Command {
    return Command::new("add")
        .about("Add authentication libraries and set things up automatically")
        .arg(
            Arg::new("add_commands")
                .help("List of authentication types to add (e.g., jwt, session, next-auth, etc.)")
                .value_parser(clap::builder::EnumValueParser::<AddVariant>::new())
                .num_args(1..)
                .required(true),
        );
}

pub fn handle_add(matches: &clap::ArgMatches) {
    if let Some(auth_types) = matches.get_many::<AddVariant>("add_commands") {
        for auth_type in auth_types {
            match auth_type {
                AddVariant::Env => add_env(),
                AddVariant::Jwt => println!("soup"),
                AddVariant::Session => println!("soup"),
                AddVariant::NextAuth => println!("soup"),
                AddVariant::PassportJwt => println!("soup"),
                AddVariant::PassportSession => println!("soup"),
                AddVariant::Firebase => println!("soup"),
            }
        }
    }
}

fn add_env() {
    if let Err(e) = create_file_in_current_dir(".env", "") {
        panic!("Error creating .env file: {}", e);
    };
    // add .env file
    // check for gitignore if not create one
    // add .env to gitignore
    // check if rust or node
    // check if exact same as template if yes then modify
    // cargo add dotenv or npm i dotenv
    // modify server.ts or main.ts
}

fn add_logger() {
    // check if rust or node
    // check if exact same as template if yes then modify
    // cargo add (loggerlib) or npm i pino
    // add logger.ts/js
}

fn create_file_in_current_dir(filename: &str, content: &str) -> io::Result<()> {
    let current_dir = env::current_dir()?;
    let file_path = current_dir.join(Path::new(filename));
    let mut file = File::create(&file_path)?;
    file.write_all(content.as_bytes())?;

    println!("File created at: {:?}", file_path);
    Ok(())
}
