use crate::cli::commands::img::sub_commands::ImgSubcommands;
use crate::error::Result;
use crate::state::AppContext;

use super::args::ImgCommand;

impl ImgCommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        log::trace!("Img Command Called");
        // time the execution

        if let Some(subcommand) = &self.subcommands {
            match subcommand {
                ImgSubcommands::Gen(ref gen) => gen.execute(),
                ImgSubcommands::Get => println!("get"),
            }
        } else {
        }

        // log successful
        // log time of execution
        Ok(())
    }
}
