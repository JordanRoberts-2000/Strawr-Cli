use clap::{builder, Arg, Command};
use enums::{NodeVariants, TemplateCommand};
use std::{env, fs, io, path::Path, process::Command as CommandLine};

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
                            install_node_dependencies();
                        } else {
                            if let Err(e) = copy_template("node/vanilla") {
                                panic!("Error generating Node template: {}", e);
                            };
                            install_node_dependencies();
                        }
                    }
                    NodeVariants::Express => {
                        if typescript_enabled {
                            if let Err(e) = copy_template("node/typescript/express") {
                                panic!("Error generating Node template: {}", e);
                            };
                            install_node_dependencies();
                        } else {
                            if let Err(e) = copy_template("node/express") {
                                panic!("Error generating Node template: {}", e);
                            };
                            install_node_dependencies();
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

fn copy_all(src: &str, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(path.file_name().unwrap());

        if path.is_dir() {
            if path.file_name().unwrap() == "node_modules" {
                continue;
            }
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

pub fn install_node_dependencies() {
    let output = CommandLine::new("npm")
        .arg("install") // Set the directory where npm install should run
        .output()
        .expect("Failed to install npm dependencies");

    if output.status.success() {
        println!("Dependencies installed successfully");
    } else {
        println!("Error installing dependencies: {:?}", output);
    }
}
