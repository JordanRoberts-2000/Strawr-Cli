// use helpers::handle_optimise::handle_optimization;
use log::debug;

use super::args::{ImgCommands, ImgSubcommands};

pub mod helpers;

impl ImgCommands {
    pub fn handle_command(&self) {
        debug!("Img command triggered");
        debug!("Path: {:?}", self.path);
        debug!("Subcommand: {:?}", self.subcommands);
        debug!("Output: {:?}", self.output);
        if self.subcommands.is_some() && !self.ignore {
            // check if valid folder or file exists in cwd
            // if yes handle_optimization()
            // warn that has happened, suggest --ignore
            println!("i want an egg");
            return;
        }

        if let Some(ImgSubcommands::Gen { .. }) = self.subcommands {
            return println!("Handle_gen({:?})", self.subcommands);
        }

        if let Some(ImgSubcommands::Get { .. }) = self.subcommands {
            return println!("Handle_get({:?})", self.subcommands);
        }

        if let Some(path) = &self.path {
            // handle_optimization(path, &self.output);
        }

        // return help
    }
}
