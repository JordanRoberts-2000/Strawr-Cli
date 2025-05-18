mod core;
mod enums;
mod impls {
    mod confirm;
    mod search;
    mod select;
    mod text;
}

pub use {
    core::MockInputRepo,
    enums::{MockInput, MockInputCall},
};
