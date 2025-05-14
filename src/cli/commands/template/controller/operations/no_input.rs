// // todo: reactor this so it takes a function
// // handle_no_input(|| self.handle_delete())

// use crate::{
//     services::editor_launcher::Editor,
//     template::{TemplateContext, TemplateController, TemplateError},
// };

// impl TemplateController {
//     pub fn handle_no_input(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
//         if self.service.has_templates()? {
//             return self.handle_select_and_inject(&ctx);
//         }

//         self.handle_no_templates(&ctx.editor)
//     }

//     fn handle_select_and_inject(&self, ctx: &TemplateContext) -> Result<(), TemplateError> {
//         let template = self.select_template()?;

//         if self.service.has_variants(&template)? {
//             let variant = self.select_variant_including_default(&template)?;
//             return self.inject_template_files(&variant.path, &ctx.output);
//         }

//         self.inject_template_files(&template.default_path(), &ctx.output)
//     }

//     fn handle_no_templates(&self, editor: &Editor) -> Result<(), TemplateError> {
//         if self.view.no_templates()? {
//             let input = self.prompt_template_name()?;
//             self.create_template(&input, &editor)?;
//         }

//         Ok(())
//     }
// }
