pub mod service;
mod impls {
    mod template {
        mod create;
        mod delete;
        mod does_not_exist;
        mod exists;
        mod get;
        mod has_variants;
        mod rename;
    }
    mod variant {
        mod create;
        mod delete;
        mod does_not_exist;
        mod exists;
        mod get;
        mod rename;
    }
    mod has_templates;
    mod init_template_folder;
}

pub(crate) use service::TemplateService;
