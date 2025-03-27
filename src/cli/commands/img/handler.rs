use crate::{
    cli::commands::img::{
        command::args::ImgCommand,
        sub_commands::{gen::args::Gen, get::args::Get, ImgSubcommands},
    },
    error::Result,
    state::AppContext,
};

pub struct Img {}

impl Img {
    pub fn execute(args: &ImgCommand, ctx: &AppContext) -> Result<()> {
        log::trace!("Img Command Called");

        if let Some(subcommand) = &args.subcommands {
            match subcommand {
                ImgSubcommands::Gen(args) => Gen::execute(args, ctx)?,
                ImgSubcommands::Get(args) => Get::execute(args, ctx)?,
            }
        } else {
            ImgCommand::execute(args, ctx)?;
        }

        Ok(())
    }
}
