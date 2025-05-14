pub mod service;
mod impls {
    mod create_template;
    mod delete_template;
    mod delete_variant;
    mod get_existing_template;
    mod get_existing_variant;
    mod get_templates;
    mod get_variants;
    mod has_templates;
    mod has_variants;
    mod init_template_folder;
}

pub(crate) use service::TemplateService;
