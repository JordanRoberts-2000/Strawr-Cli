mod core;
mod error;
mod impls {
    mod confirm;
    mod multi_select;
    mod search;
    mod select;
    mod text;
}

pub use {core::UserInputRepo, error::UserInputError};
