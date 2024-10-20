use clap::Command;
use commands::{
    add::{add_command, handle_add},
    font::{font_command, handle_font},
    img::{handle_img, img_command},
    template::{command::template_command, handle_template},
};
use dotenv::from_path;

mod commands;
mod utils;

fn main() {
    load_env();
    let main_command = Command::new("CLI Tool")
        .version("1.0")
        .about("A tool for handling images, fonts, templates, and more")
        .subcommand(img_command())
        .subcommand(font_command())
        .subcommand(template_command())
        .subcommand(add_command())
        .get_matches();
    match main_command.subcommand() {
        Some(("img", sub_matches)) => handle_img(sub_matches),
        Some(("font", sub_matches)) => handle_font(sub_matches),
        Some(("template", sub_matches)) => handle_template(sub_matches),
        Some(("add", sub_matches)) => handle_add(sub_matches),
        _ => println!("Invalid command. Available commands: img, font, template, add"),
    }
    // let api_key = env::var("OPENAI_API_KEY").expect("API key not found in .env file");
}

fn load_env() {
    if let Err(err) = from_path("/Users/jordanroberts/Documents/dev/Projects/main/rustCli/.env") {
        panic!("Error loading .env file: {}", err);
    }
}
