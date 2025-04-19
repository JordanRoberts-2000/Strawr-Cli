use gen::args::Gen;
use get::args::Get;

use crate::state::AppContext;

use super::ImgError;

pub mod gen;
pub mod get;

#[derive(clap::Subcommand, Debug)]
pub enum ImgSubcommand {
    Get(Get),
    Gen(Gen),
}

impl ImgSubcommand {
    pub fn execute(&self, ctx: &AppContext) -> Result<(), ImgError> {
        match self {
            ImgSubcommand::Gen(cmd) => cmd.execute(ctx)?,
            ImgSubcommand::Get(cmd) => cmd.execute(ctx)?,
        }

        Ok(())
    }
}
