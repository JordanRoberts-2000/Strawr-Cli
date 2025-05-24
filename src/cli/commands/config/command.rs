use strum::IntoEnumIterator;

use crate::{services::editor_launcher::Editor, utils::Keyring, CliContext};

use super::{enums::ConfigOption, ConfigCommandError};

#[derive(clap::Parser, Debug)]
pub struct ConfigCommand {
    #[arg(short, long, ignore_case = true)]
    pub editor: Option<Editor>,
    #[arg(
      short = 'o',
      long = "open",
      action = clap::ArgAction::SetTrue,
      help = "Open the configuration file and exit"
    )]
    pub open: bool,
}

impl ConfigCommand {
    pub fn execute(&self, ctx: &CliContext) -> Result<(), ConfigCommandError> {
        let editor = self.editor.as_ref().unwrap_or(&ctx.config.default_editor);

        if self.open {
            ctx.service
                .init_launch_editor()
                .launch_editor(editor, &ctx.config_path)?;
            return Ok(());
        }

        let msg = "Select option:";
        let options: Vec<ConfigOption> = ConfigOption::iter().collect();
        let option = ctx.service.init_prompt().select(&options, msg)?;

        match option {
            ConfigOption::OpenConfig => {
                ctx.service
                    .init_launch_editor()
                    .launch_editor(editor, &ctx.config_path)?;
            }
            ConfigOption::UpdateApiKey => ctx.service.init_keyring().update(Keyring::OpenAiKey)?,
            ConfigOption::UpdatePassword => ctx.service.init_keyring().update(Keyring::Password)?,
        };

        Ok(())
    }
}
