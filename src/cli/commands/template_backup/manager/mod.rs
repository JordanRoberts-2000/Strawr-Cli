mod manager;
mod impls {
    mod inject;
    mod template {
        mod create;
        mod delete;
        mod has_templates;
        mod new;
        mod rename;
        mod select;
        mod select_all;
    }
    mod variant {
        mod create;
        mod delete;
        mod has_variants;
        mod new;
        mod rename;
        mod select;
    }
}

pub(crate) use manager::TemplateManager;
