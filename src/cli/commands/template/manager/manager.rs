use std::path::Path;

use crate::{traits::ToManager, CliService};

pub struct TemplateManager<'a> {
    pub(super) service: &'a CliService,
    pub templates_path: &'a Path,
}

impl<'a> TemplateManager<'a> {
    pub fn new(service: &'a CliService, templates_path: &'a Path) -> Self {
        Self {
            service,
            templates_path,
        }
    }

    pub fn from<T>(ctx: &'a T) -> Self
    where
        T: ToManager<'a, Self> + ?Sized,
    {
        ctx.to_manager()
    }
}
