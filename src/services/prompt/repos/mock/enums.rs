pub enum MockInput {
    Confirm(bool),
    Text(String),
    Select(String),
    Search(String),
}

pub enum MockInputCall {
    Confirm { msg: String },
    Text { msg: String },
    Select { options: Vec<String>, msg: String },
    Search { options: Vec<String>, msg: String },
}
