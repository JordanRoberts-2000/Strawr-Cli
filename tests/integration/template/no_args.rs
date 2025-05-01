use std::env;

use strawr::{
    cli::commands::template::TemplateCommand,
    constants::CONFIG_HOME_ENV,
    state::AppContext,
    utils::input::{Input, TestInput},
};
use tempfile::tempdir;

#[test]
fn test_template_command_with_no_args() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    env::set_var(CONFIG_HOME_ENV, temp_dir.path());

    let ctx = AppContext::initialize(&false).expect("AppContext failed to initialize");

    dbg!(&ctx.storage_dir);

    let cmd = TemplateCommand {
        template: None,
        subcommand: None,
        variant: None,
        backend: None,
        frontend: None,
    };

    // let inputs = TestInput::new(vec![Input::Confirm(false)]);
    // let result = cmd.execute(&ctx, &inputs);
    // assert!(result.is_ok(), "Expected OK when no arguments are provided");

    // let inputs = TestInput::new(vec![
    //     Input::Confirm(true),
    //     Input::Text("Initial_Template".to_string()),
    // ]);

    // let result = cmd.execute(&ctx, &inputs);
    // assert!(result.is_ok(), "Expected OK when no arguments are provided");
}
