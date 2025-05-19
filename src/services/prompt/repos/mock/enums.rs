pub enum MockInput {
    Confirm(bool),
    Text(String),
    Password(String),
    Select(String),
    Search(String),
    Checklist(Vec<String>),
}

#[derive(Debug, PartialEq)]
pub enum MockInputCall {
    Confirm {
        msg: String,
    },
    Text {
        msg: String,
    },
    Password {
        msg: String,
    },
    Select {
        options: Vec<String>,
        msg: String,
    },
    Search {
        options: Vec<String>,
        msg: String,
    },
    Checklist {
        options: Vec<String>,
        defaults: Vec<usize>,
        msg: String,
    },
}
