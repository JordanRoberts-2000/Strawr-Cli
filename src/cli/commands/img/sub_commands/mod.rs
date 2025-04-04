use gen::args::Gen;
use get::args::Get;

use crate::state::AppContext;

use super::ImgError;

pub mod gen;
pub mod get;

#[derive(clap::Subcommand, Debug)]
pub enum ImgSubcommands {
    Get(Get),
    Gen(Gen),
}

impl ImgSubcommands {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        match self {
            ImgSubcommands::Gen(cmd) => cmd.execute(ctx)?,
            ImgSubcommands::Get(cmd) => cmd.execute(ctx)?,
        }

        Ok(())
    }
}
