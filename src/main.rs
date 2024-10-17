use clap::Command;
use commands::{
    font::{font_command, handle_font},
    img::{handle_img, img_command},
    template::{handle_template, template_command},
};
use dotenv::dotenv;
use std::env;

mod commands;

fn main() {
    dotenv().ok();
    let main_command = Command::new("CLI Tool")
        .version("1.0")
        .about("A tool for handling images, fonts, templates, and more")
        .subcommand(img_command())
        .subcommand(font_command())
        .subcommand(template_command())
        .subcommand(Command::new("add").about("Handle add"))
        .get_matches();
    match main_command.subcommand() {
        Some(("img", sub_matches)) => handle_img(sub_matches),
        Some(("font", sub_matches)) => handle_font(sub_matches),
        Some(("template", sub_matches)) => handle_template(sub_matches),
        Some(("add", _)) => println!("Handle add"),
        _ => println!("Invalid command. Available commands: img, font, template, add"),
    }
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found in .env file");
}
