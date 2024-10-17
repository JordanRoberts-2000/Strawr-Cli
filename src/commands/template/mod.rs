use clap::{builder, Arg, Command};
use enums::{NodeVariants, TemplateCommand};
use std::{env, fs, io};

use crate::utils::{copy_all, run_command};

mod enums;

pub fn template_command() -> Command {
    return Command::new("template")
        .about("Handle templates")
        .arg(
            Arg::new("template_type")
                .help("Specify the template type")
                .value_parser(builder::EnumValueParser::<TemplateCommand>::new())
                .required(true),
        )
        .arg(
            Arg::new("variant")
                .short('v')
                .long("variant")
                .help("Specify a variant for the template (e.g., express, typescript, etc.)")
                .value_parser(builder::EnumValueParser::<NodeVariants>::new()),
        )
        .arg(
            Arg::new("typescript")
                .short('t')
                .long("typescript")
                .help("Specify a variant for the template (e.g., express, typescript, etc.)")
                .value_parser(builder::BoolValueParser::new()),
        );
}

pub fn handle_template(matches: &clap::ArgMatches) {
    if let Some(template_type) = matches.get_one::<TemplateCommand>("template_type") {
        match template_type {
            TemplateCommand::React => println!("Generating React template..."),
            TemplateCommand::Node => {
                let variant = matches
                    .get_one::<NodeVariants>("variant")
                    .unwrap_or(&NodeVariants::Express);
                let typescript_enabled = *matches.get_one::<bool>("typescript").unwrap_or(&true);
                match variant {
                    NodeVariants::Plain => {
                        if typescript_enabled {
                            if let Err(e) = copy_template("node/typescript/vanilla") {
                                panic!("Error generating Node template: {}", e);
                            };
                            if let Err(e) = run_command("npm i") {
                                panic!("Error running npm i: {}", e);
                            };
                        } else {
                            if let Err(e) = copy_template("node/vanilla") {
                                panic!("Error generating Node template: {}", e);
                            };
                            if let Err(e) = run_command("npm i") {
                                panic!("Error running npm i: {}", e);
                            };
                        }
                    }
                    NodeVariants::Express => {
                        if typescript_enabled {
                            if let Err(e) = copy_template("node/typescript/express") {
                                panic!("Error generating Node template: {}", e);
                            };
                            if let Err(e) = run_command("npm i") {
                                panic!("Error running npm i: {}", e);
                            };
                        } else {
                            if let Err(e) = copy_template("node/express") {
                                panic!("Error generating Node template: {}", e);
                            };
                            if let Err(e) = run_command("npm i") {
                                panic!("Error running npm i: {}", e);
                            };
                        }
                    }
                    _ => println!("not a valid node variant"),
                }
            }
            TemplateCommand::Fullstack => println!("Generating Fullstack template..."),
            TemplateCommand::Rust => {
                if let Err(e) = copy_template("rust") {
                    println!("Error generating Rust template: {}", e);
                };
            }
            TemplateCommand::Go => println!("Generating Go template..."),
            TemplateCommand::Next => println!("Generating Next template..."),
        }
    }
}

fn copy_template(template_name: &str) -> io::Result<()> {
    let project_root = env!("CARGO_MANIFEST_DIR"); // Compile-time project root
    let template_path = format!("{}/assets/templates/{}", project_root, template_name);
    let current_dir = env::current_dir()?; // Get the current directory where the CLI is executed

    if !fs::metadata(&template_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Template path not found: {}", template_path),
        ));
    }
    copy_all(&template_path, &current_dir)?;

    Ok(())
}
