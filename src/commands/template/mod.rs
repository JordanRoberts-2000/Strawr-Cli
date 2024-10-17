use clap::{builder, Arg, Command};
use enums::TemplateCommand;
use std::{env, fs, io, path::Path};

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

    let template_path = format!("asses/templates/{}", template_name);

    if !fs::metadata(&template_path).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Template path not found: {}", template_path),
        ));
    }
    copy_all(&template_path, &current_dir)?;

    Ok(())
}

fn copy_all(src: &str, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(path.file_name().unwrap());

        if path.is_dir() {
            // If it's a directory, create the directory in the destination and recurse
            fs::create_dir_all(&dest_path)?;
            copy_all(path.to_str().unwrap(), &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
            println!("Copied file to: {:?}", dest_path);
        }
    }
    Ok(())
}
