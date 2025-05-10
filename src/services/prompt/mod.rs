mod error;
pub mod traits;
mod types;
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

pub(crate) use traits::CliInput;
pub use {
    error::PromptError, input::cli::input::UserInput, input::test::input::TestInput, types::Input,
};
