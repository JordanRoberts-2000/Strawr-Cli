use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found in .env file");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "img" => handle_img(&args),
            "font" => handle_font(&args),
            "template" => println!("handle templates"),
            "add" => println!("handle add"),
            _ => println!("Invalid command"),
        }
    } else {
        println!("Commands: img, font, template, add,")
    }
}

fn handle_img(args: &Vec<String>) {
    if args.len() > 2 {
        match args[2].as_str() {
            "gen" => println!("generate image"),
            "opt" => println!("optimize image"),
            _ => println!("invalid input, try: gen, opt"),
        }
    } else {
        println!("Commands: gen, opt")
    }
}

fn handle_font(args: &Vec<String>) {
    if args.len() > 2 {
        match args[2].as_str() {
            "gen" => println!("generate font"),
            "opt" => println!("optimize font"),
            _ => println!("invalid input, try: gen, opt"),
        }
    } else {
        println!("Commands: gen, opt")
    }
}
