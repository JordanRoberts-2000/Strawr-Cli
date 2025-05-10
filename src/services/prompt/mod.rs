mod enums;
mod error;
pub mod traits;
mod input {
    pub(crate) mod cli {
        pub(crate) mod input;
        mod impls {
            mod confirm;
            mod search;
            mod select;
            mod text;
        }
    }
    pub(crate) mod test {
        pub(crate) mod input;
        mod impls {
            mod confirm;
            mod search;
            mod select;
            mod text;
        }
    }
}

pub use {
    enums::Input, error::PromptError, input::cli::input::UserInput, input::test::input::TestInput,
};
