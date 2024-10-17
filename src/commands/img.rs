use clap::{builder, Arg, Command, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Size {
    Sm,
    Md,
    Lg,
}

// Define an enum for aspect ratio with possible values
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Aspect {
    Square,
    Wide,
    Tall,
}

pub fn img_command() -> Command {
    return Command::new("img")
        .about("Image operations")
        .subcommand(
            Command::new("gen")
                .about("Generate an image")
                .arg(Arg::new("prompt").short('p').long("prompt").required(true))
                .arg(
                    Arg::new("size")
                        .short('s')
                        .long("size")
                        .value_parser(builder::EnumValueParser::<Size>::new()),
                )
                .arg(
                    Arg::new("aspect")
                        .short('a')
                        .long("aspect")
                        .value_parser(builder::EnumValueParser::<Aspect>::new()),
                ),
        )
        .subcommand(Command::new("opt").about("Optimize an image"));
}

pub fn handle_img(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("gen", sub_matches)) => {
            let prompt = sub_matches.get_one::<String>("prompt").expect("required");
            let size = sub_matches.get_one::<Size>("size").unwrap_or(&Size::Lg);
            let aspect = sub_matches
                .get_one::<Aspect>("aspect")
                .unwrap_or(&Aspect::Square);

            // Print the arguments
            println!("Generating image with:");
            println!("  Prompt: {}", prompt);
            println!("  Size: {:?}", size);
            println!("  Aspect Ratio: {:?}", aspect);
        }
        Some(("opt", _)) => println!("Optimizing image..."),
        _ => println!("Invalid image command. Use 'gen' or 'opt'."),
    }
}
