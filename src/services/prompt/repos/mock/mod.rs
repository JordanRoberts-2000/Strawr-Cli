mod core;
mod enums;
mod impls {
    mod confirm;
    mod multi_select;
    mod password;
    mod search;
    mod select;
    mod text;
}

pub use {
    core::MockInputRepo,
    enums::{MockInput, MockInputCall},
};
