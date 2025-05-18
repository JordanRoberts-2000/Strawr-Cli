mod core;
mod error;
mod impls {
    mod confirm;
    mod search;
    mod select;
    mod text;
}

pub use {core::UserInputRepo, error::UserInputError};
