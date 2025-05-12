pub mod service;
mod impls {
    mod create_template;
    mod delete_template;
    mod get_templates;
    mod get_variants;
    mod has_templates;
    mod init_template_folder;
}

pub(crate) use service::TemplateService;
