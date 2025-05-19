use strawr::services::prompt::{user::UserInputRepo, PromptService};

fn main() {
    let fruits = vec![
        "apple",
        "peach",
        "pears",
        "plums",
        "bananas",
        "strawberries",
    ];

    let prompt = PromptService::new(UserInputRepo);
    match prompt.search(&fruits, "Choose a fruit:\n") {
        Ok(input) => println!("{input}"),
        Err(e) => eprintln!("Error: {e}"),
    };
}
