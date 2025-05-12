use std::path::Path;

use crate::{services::editor_launcher::Editor, CliContext};

use super::{
    constants::TEMPLATES_FOLDER_NAME, models::Template, service::TemplateService,
    view::TemplateView, TemplateCommand, TemplateContext, TemplateError,
};

pub struct TemplateController {
    service: TemplateService,
    view: TemplateView,
}

impl TemplateController {
    pub fn new(ctx: &CliContext) -> Self {
        let templates_path = ctx.storage_dir.join(TEMPLATES_FOLDER_NAME);
        Self {
            service: TemplateService::new(&templates_path),
            view: TemplateView::new(&ctx.config.mute),
        }
    }

    pub fn execute(&self, args: &TemplateCommand, ctx: &CliContext) -> Result<(), TemplateError> {
        self.service.init_templates_folder()?;

        if let Some(subcommand) = args.subcommand {
            // todo handle subcommmands
        }

        let ctx = TemplateContext::new(args, ctx);

        // if let Some(template) = &self.template {
        //     return handle_template_input(&ctx, &manager, template);
        // }

        if ctx.backend.is_some() || ctx.frontend.is_some() {
            return self.handle_stack_flags(&ctx);
        }

        self.handle_no_input(&ctx)?;
        Ok(())
    }

    pub fn handle_stack_flags(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
        let stacks = [
            (ctx.backend, ctx.backend_folder_title),
            (ctx.frontend, ctx.frontend_folder_title),
        ];

        for (maybe_override, folder_title) in stacks {
            if let Some((raw_template, raw_variant)) = maybe_override {
                let template = self.service.check_template_exists(raw_template)?;

                let target_dir = ctx.output.join(folder_title);
                if !target_dir.exists()? {
                    self.service.fs.create_dir_all(&target_dir)?;
                }

                if let Some(v) = raw_variant {
                    let variant = self.service.check_variant_exists(&template, v)?;
                    self.handle_inject_template_files(&variant.path, &target_dir)?;
                    break;
                }

                self.handle_inject_template_files(&template.path, &target_dir)?;
            }
        }

        Ok(())
    }

    pub fn create_template(&self, str: &str, editor: &Editor) -> Result<Template, TemplateError> {
        let template = self.service.create_template(str)?;
        self.view.template_created(&template);
        self.service
            .launch_editor(&editor, &template.default_path())?;

        Ok(template)
    }

    pub fn handle_no_templates(&self, editor: &Editor) {
        if self.view.no_templates()? {
            let input = self.view.enter_template_name()?;
            self.create_template(&input, &editor)?;
        }
    }

    pub fn handle_select_and_inject(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
        let template = self.select_template()?;
        let variants = self.service.get_variants(&template)?;

        if variants.len() > 1 {
            let input = self.view.select_variant(&variants)?;
            return self
                .service
                .inject(&template.path.join(&input), &ctx.output);
        }

        self.service.inject(&template.default_path(), &ctx.output)
    }

    pub fn handle_no_input(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
        if self.service.has_templates()? {
            return self.handle_select_and_inject(&ctx);
        }

        self.handle_no_templates(&ctx.editor);
        Ok(())
    }

    pub fn select_template(&self) -> Result<Template, TemplateError> {
        let templates = self.service.get_templates()?;
        let input = self.view.select_template(&templates)?;
        let template = Template::new(&input, &self.service.templates_path.join(&input))?;

        Ok(template)
    }

    pub fn handle_inject_template_files(
        &self,
        path: &Path,
        output: &Path,
    ) -> Result<(), TemplateError> {
        if !self.service.fs.dir_empty(&output)? && self.view.output_not_empty_warning()? {
            return Ok(());
        }

        self.service.fs.copy_dir_contents(&path, &output)?;
        Ok(())
    }

    pub fn delete_template(&self, template: &Template) -> Result<(), TemplateError> {
        if !self.view.delete_confirmation(&template)? {
            return Ok(());
        }

        self.service.delete_template(&template)?;
        self.view.template_deleted(&template);

        Ok(())
    }
}
