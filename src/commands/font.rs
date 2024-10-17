use clap::Command;

pub fn font_command() -> Command {
    return Command::new("font")
        .about("Font operations")
        .subcommand(Command::new("gen").about("Generate a font"))
        .subcommand(Command::new("opt").about("Optimize a font"));
}

pub fn handle_font(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("gen", _)) => println!("Generating font..."),
        Some(("opt", _)) => println!("Optimizing font..."),
        _ => println!("Invalid font command. Use 'gen' or 'opt'."),
    }
}
