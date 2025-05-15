pub mod core;
mod impls {
    mod create_template;
    mod delete_template;
    mod edit_template;
    mod inject_files;
    mod rename_template;
}

pub use core::TemplateResolver;
