use strawr::services::prompt::PromptService;

fn main() {
    let fruits = vec![
        "apple",
        "peach",
        "pears",
        "plums",
        "bananas",
        "strawberries",
    ];

    let prompt = PromptService::new();
    match prompt.search(&fruits, "Choose a fruit:\n") {
        Ok(input) => println!("{input}"),
        Err(e) => eprintln!("Error: {e}"),
    };
}
