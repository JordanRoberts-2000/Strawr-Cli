pub mod core;
pub mod impls {
    mod delete_template;
    mod inject_files;
}

pub use core::TemplateResolver;
