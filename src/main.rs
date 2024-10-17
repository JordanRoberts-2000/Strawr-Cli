use clap::Command;
use commands::img::{handle_img, img_command};
use dotenv::dotenv;
use std::env;

mod commands;

fn main() {
    dotenv().ok();
    let font_command = Command::new("font")
        .about("Font operations")
        .subcommand(Command::new("gen").about("Generate a font"))
        .subcommand(Command::new("opt").about("Optimize a font"));
    let main_command = Command::new("CLI Tool")
        .version("1.0")
        .about("A tool for handling images, fonts, templates, and more")
        .subcommand(img_command())
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

fn handle_font(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("gen", _)) => println!("Generating font..."),
        Some(("opt", _)) => println!("Optimizing font..."),
        _ => println!("Invalid font command. Use 'gen' or 'opt'."),
    }
}
