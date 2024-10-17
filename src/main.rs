use clap::{builder, Arg, Command, ValueEnum};
use dotenv::dotenv;
use std::env;

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

fn main() {
    dotenv().ok();
    let img_command = Command::new("img")
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
    let font_command = Command::new("font")
        .about("Font operations")
        .subcommand(Command::new("gen").about("Generate a font"))
        .subcommand(Command::new("opt").about("Optimize a font"));
    let main_command = Command::new("CLI Tool")
        .version("1.0")
        .about("A tool for handling images, fonts, templates, and more")
        .subcommand(img_command)
        .subcommand(font_command)
        .subcommand(Command::new("template").about("Handle templates"))
        .subcommand(Command::new("add").about("Handle add"))
        .get_matches();
    match main_command.subcommand() {
        Some(("img", sub_matches)) => handle_img(sub_matches),
        Some(("font", sub_matches)) => handle_font(sub_matches),
        Some(("template", _)) => println!("Handle templates"),
        Some(("add", _)) => println!("Handle add"),
        _ => println!("Invalid command. Available commands: img, font, template, add"),
    }
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found in .env file");
}

fn handle_img(matches: &clap::ArgMatches) {
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

fn handle_font(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("gen", _)) => println!("Generating font..."),
        Some(("opt", _)) => println!("Optimizing font..."),
        _ => println!("Invalid font command. Use 'gen' or 'opt'."),
    }
}
