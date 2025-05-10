pub enum Input {
    Confirm(bool),
    Text(String),
    Select(String),
    SelectWithoutFilter(String),
}
