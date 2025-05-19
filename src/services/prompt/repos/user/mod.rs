mod core;
mod error;
mod impls {
    mod confirm;
    mod multi_select;
    mod password;
    mod search;
    mod select;
    mod text;
}

pub use {core::UserInputRepo, error::UserInputError};
