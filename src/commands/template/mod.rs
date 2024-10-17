use clap::{builder, Arg, Command};
use enums::TemplateCommand;
use std::{env, fs, io};

mod enums;

pub fn template_command() -> Command {
    return Command::new("template").about("Handle templates").arg(
        Arg::new("template_type")
            .help("Specify the template type")
            .value_parser(builder::EnumValueParser::<TemplateCommand>::new())
            .required(true),
    );
}

pub fn handle_template(matches: &clap::ArgMatches) {
    if let Some(template_type) = matches.get_one::<TemplateCommand>("template_type") {
        match template_type {
            TemplateCommand::React => println!("Generating React template..."),
            TemplateCommand::Node => {
                if let Err(e) = generate_template_node() {
                    println!("Error generating Node template: {}", e);
                }
            }
            TemplateCommand::Fullstack => println!("Generating Fullstack template..."),
            TemplateCommand::Rust => println!("Generating Rust template..."),
            TemplateCommand::Go => println!("Generating Go template..."),
            TemplateCommand::Next => println!("Generating Next template..."),
        }
    }
}

pub fn generate_template_node() -> io::Result<()> {
    copy_template("node/vanilla")
}

fn copy_template(template_name: &str) -> io::Result<()> {
    // Get the current directory where the CLI is executed
    let current_dir = env::current_dir()?;

    // Path to the template files in the assets/templates directory
    let template_path = format!("assets/templates/{}", template_name);

    // Check if the template path exists
    if !fs::metadata(&template_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Template path not found: {}", template_path),
        ));
    }

    // Copy the contents of the template directory to the current directory
    for entry in fs::read_dir(template_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap();
            let destination = current_dir.join(file_name);
            fs::copy(&path, &destination)?;
            println!("Copied file to: {:?}", destination); // Debug output
        }
    }
    Ok(())
}
