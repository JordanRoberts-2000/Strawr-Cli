pub mod core;
pub mod impls {
    mod delete_template;
    mod inject_files;
    mod rename_template;
}

pub use core::TemplateResolver;
