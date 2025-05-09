use std::fs;

use strawr::CliContext;
use tempfile::tempdir;

// #[test]
// fn test_template_command_with_no_args_empty_templates_user_declines() {
//     let storage_dir = tempfile::tempdir()
//         .expect("Failed to create temp dir")
//         .path()
//         .to_path_buf();

//     let ctx = AppContext {
//         input: Box::new(TestInput::from(vec![Input::Confirm(false)])),
//         editor: Box::new(TestEditorLauncher::new(false)),
//         storage_dir: storage_dir.clone(),
//         debug: true,
//         config: Config::default(),
//     };

//     let cmd = TemplateCommand {
//         template: None,
//         subcommand: None,
//         variant: None,
//         backend: None,
//         frontend: None,
//         editor: None,
//     };

//     let result = cmd.execute(&ctx);
//     assert!(
//         result.is_ok(),
//         "Expected OK when no arguments are provided, got error: {:?}",
//         result.unwrap_err()
//     );
// }

// #[test]
// fn test_template_command_with_no_args_empty_templates_user_accepts() {
//     let storage_dir = tempfile::tempdir()
//         .expect("Failed to create temp dir")
//         .path()
//         .to_path_buf();

//     let ctx = AppContext {
//         input: Box::new(TestInput::from(vec![
//             Input::Confirm(true),
//             Input::Text("react".to_string()),
//         ])),
//         editor: Box::new(TestEditorLauncher::new(false)),
//         storage_dir: storage_dir.clone(),
//         debug: true,
//         config: Config::default(),
//     };

//     let cmd = TemplateCommand {
//         template: None,
//         subcommand: None,
//         variant: None,
//         backend: None,
//         frontend: None,
//         editor: None,
//     };

//     let result = cmd.execute(&ctx);
//     assert!(
//         result.is_ok(),
//         "Expected OK when no arguments are provided, got error: {:?}",
//         result.unwrap_err()
//     );

//     let expected_path = storage_dir
//         .join("templates")
//         .join("react")
//         .join(DEFAULT_FOLDER);
//     assert!(
//         expected_path.exists() && expected_path.is_dir(),
//         "Expected template path {:?} to be created, but it does not exist",
//         expected_path
//     );
// }

// #[test]
// fn test_template_command_with_no_args_templates_exist() {
//     let storage_dir = tempfile::tempdir()
//         .expect("Failed to create temp dir")
//         .path()
//         .to_path_buf();

//     let ctx = AppContext {
//         input: Box::new(TestInput::from(vec![Input::Select("react".to_string())])),
//         editor: Box::new(TestEditorLauncher::new(false)),
//         storage_dir: storage_dir.clone(),
//         debug: true,
//         config: Config::default(),
//     };

//     let cmd = TemplateCommand {
//         template: None,
//         subcommand: None,
//         variant: None,
//         backend: None,
//         frontend: None,
//         editor: None,
//     };

//     let template_path = storage_dir
//         .join("templates")
//         .join("react")
//         .join(DEFAULT_FOLDER);
//     fs::create_dir_all(&template_path).expect("Failed to create template directory structure");

//     let tmp_current_dir = tempdir().expect("Failed to create temporary directory");
//     std::env::set_current_dir(&tmp_current_dir)
//         .expect("Failed to set current working directory to temp dir");

//     let result = cmd.execute(&ctx);
//     assert!(
//         result.is_ok(),
//         "Expected OK when no arguments are provided, got error: {:?}",
//         result.unwrap_err()
//     );
// }
