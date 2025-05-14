// #[cfg(test)]
// pub mod test_utils {
//     use crate::{
//         cli::commands::template::command::manager::TemplateManager,
//         config::Config,
//         state::AppContext,
//         utils::{
//             editor::TestEditorLauncher,
//             input::{Input, TestInput},
//             Editor,
//         },
//     };

//     pub fn create_test_manager(inputs: Vec<Input>) -> TemplateManager<'static> {
//         let temp = tempfile::tempdir().expect("Failed to create temp dir");
//         let storage_dir = temp.path().to_path_buf();

//         let ctx = Box::leak(Box::new(AppContext {
//             input: Box::new(TestInput::from(inputs)),
//             editor: Box::new(TestEditorLauncher::new(false)),
//             storage_dir: storage_dir.clone(),
//             debug: true,
//             config: Config::default(),
//         }));

//         let manager =
//             TemplateManager::new(ctx, &Editor::VsCode).expect("Failed to create TemplateManager");

//         manager
//     }
// }
