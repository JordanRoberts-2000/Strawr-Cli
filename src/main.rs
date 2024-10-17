use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("API key not found in .env file");
    println!("{api_key}");
    println!("Hello, world!");
}
