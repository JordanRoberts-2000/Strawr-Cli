use crate::{
    cli::commands::img::{command::args::ImgCommand, sub_commands::ImgSubcommands},
    error::Result,
    state::AppContext,
};

pub struct Img {}

impl Img {
    pub fn execute(args: &ImgCommand, ctx: &AppContext) -> Result<()> {
        log::trace!("Img Command Called");

        if let Some(subcommand) = &args.subcommands {
            match subcommand {
                ImgSubcommands::Gen(_args) => println!("gen"),
                ImgSubcommands::Get => println!("get"),
            }
        } else {
            ImgCommand::execute(args, ctx)?;
        }

        Ok(())
    }
}
