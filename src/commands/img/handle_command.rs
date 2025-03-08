use crate::error::Result;
use crate::state::AppContext;

use super::{ImgCommand, ImgSubcommands};

impl ImgCommand {
    pub fn handle_command(&self, ctx: &AppContext) -> Result<()> {
        log::trace!("Img Command Called");

        if let Some(subcommand) = &self.subcommands {
            match subcommand {
                ImgSubcommands::Gen => println!("Handle_gen"),
                ImgSubcommands::Get => println!("Handle_get"),
            }
        } else {
            // println!("handle_optimize");
        }

        let path = self.validate_path()?;

        if path.is_file() {
            // check if file is a valid image format
        }

        Ok(())
    }
}
