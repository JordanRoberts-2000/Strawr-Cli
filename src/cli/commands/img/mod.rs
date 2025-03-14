use command::args::ImgCommand;
use sub_commands::ImgSubcommands;

use crate::error::Result;
use crate::state::AppContext;

pub mod command;
pub mod config;
pub mod sub_commands;

// pub use command::args::;

pub fn execute_img(args: &ImgCommand, ctx: &AppContext) -> Result<()> {
    log::trace!("Img Command Called");
    // time the execution

    if let Some(subcommand) = &args.subcommands {
        match subcommand {
            ImgSubcommands::Gen(ref gen) => gen.execute(ctx),
            ImgSubcommands::Get => println!("get"),
        }
    } else {
        ImgCommand::execute(args, ctx);
    }

    // log successful
    // log time of execution
    Ok(())
}
