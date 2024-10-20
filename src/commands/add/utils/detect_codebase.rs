use std::{fs, path::Path};

pub enum CodeBase {
    Node,
    React,
    Rust,
}

pub fn detect_codebase(code_base: CodeBase) -> bool {
    match code_base {
        CodeBase::Node => detect_node_project(),
        CodeBase::React => detect_react_project(),
        CodeBase::Rust => detect_rust_project(),
    }
}

fn detect_rust_project() -> bool {
    Path::new("Cargo.toml").exists()
}

fn detect_node_project() -> bool {
    Path::new("package.json").exists()
}

fn detect_react_project() -> bool {
    if let Ok(contents) = fs::read_to_string("package.json") {
        contents.contains("\"react\"") || contents.contains("\"react-dom\"")
    } else {
        false
    }
}
