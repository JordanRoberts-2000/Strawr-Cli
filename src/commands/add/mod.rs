use args::{env::add_env, logger::add_logger};
use clap::{builder, Arg, Command, ValueEnum};

mod args;
mod utils;

#[derive(ValueEnum, Clone)]
enum AddVariant {
    Env,
    Logger,
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
                .value_parser(builder::EnumValueParser::<AddVariant>::new())
                .num_args(1..)
                .required(true),
        );
}

pub fn handle_add(matches: &clap::ArgMatches) {
    if let Some(auth_types) = matches.get_many::<AddVariant>("add_commands") {
        for auth_type in auth_types {
            match auth_type {
                AddVariant::Env => add_env(),
                AddVariant::Logger => add_logger(),
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
