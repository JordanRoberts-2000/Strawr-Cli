pub mod service;
mod impls {
    mod template {
        mod create;
        mod delete;
        mod get;
        mod has_variants;
        mod open_in_editor;
        mod rename;
    }
    mod variant {
        mod create;
        mod delete;
        mod get;
        mod open_in_editor;
        mod rename;
    }
    mod has_templates;
    mod init_template_folder;
}

pub(crate) use service::TemplateService;
