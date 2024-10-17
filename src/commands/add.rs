use clap::{builder, Arg, Command, ValueEnum};

#[derive(ValueEnum, Clone)]
enum addVariants {
    Dotenv,
}

pub fn add_command() -> Command {
    return Command::new("template").about("Add to existing code").arg(
        Arg::new("template_type")
            .help("Specify the template type")
            .value_parser(builder::EnumValueParser::<addVariants>::new())
            .required(true),
    );
}

pub fn handle_add(matches: &clap::ArgMatches) {
    if let Some(template_type) = matches.get_one::<addVariants>("template_type") {
        match template_type {
            addVariants::Dotenv => println!("Generating React template..."),
        }
    }
}
