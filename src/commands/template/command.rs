use super::enums::{NodeVariants, TemplateCommand};
use clap::{builder, Arg, Command};

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
        );
}