use strawr::prompt::{traits::SelectPrompt, UserInput};

#[derive(Debug)]
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
    let fruits = vec![
        "apple",
        "peach",
        "pears",
        "plums",
        "bananas",
        "strawberries",
    ];

    let fruits_enum = vec![Fruit::Apple, Fruit::Banana];

    let input = UserInput;
    match input.select(&fruits, "Choose a fruit:\n") {
        Ok(input) => println!("{input}"),
        Err(e) => eprintln!("Error: {e}"),
    };
}
