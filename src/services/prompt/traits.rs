pub trait ConfirmPrompt {
    type Error;
    fn confirm(&self, msg: &str) -> Result<bool, Self::Error>;
}

pub trait SelectPrompt {
    type Error;
    fn select(&self, options: &[String], msg: &str) -> Result<String, Self::Error>;
}

pub trait SearchPrompt {
    type Error;
    fn search(&self, options: &[String], msg: &str) -> Result<String, Self::Error>;
}

pub trait TextPrompt {
    type Error;
    fn text(&self, msg: &str) -> Result<String, Self::Error>;
}
