use strawr::services::prompt::{user::UserInputRepo, PromptService};

#[derive(Debug, Clone)]
pub enum Fruit {
    Apple,
    Banana,
}
impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let fruits_enum = vec![Fruit::Apple, Fruit::Banana];

    let prompt = PromptService::new(UserInputRepo);
    match prompt.select(&fruits_enum, "Choose a fruit:\n") {
        Ok(input) => {
            match input {
                Fruit::Apple => println!("Enjoy your apple"),
                Fruit::Banana => println!("Enjoy your banana"),
            };
        }
        Err(e) => eprintln!("Error: {e}"),
    };
}
