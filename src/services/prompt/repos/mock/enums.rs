pub enum MockInput {
    Confirm(bool),
    Text(String),
    Select(String),
    Search(String),
    Checklist(Vec<String>),
}

#[derive(Debug)]
pub enum MockInputCall {
    Confirm {
        msg: String,
    },
    Text {
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
