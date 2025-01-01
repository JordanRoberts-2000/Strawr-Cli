use super::args::{ImgCommands, ImgSubcommands};

impl ImgCommands {
    pub fn handle_command(&self) {
        if self.subcommands.is_some() && !self.ignore {
            // check if valid folder or file exists in cwd
            // if yes handle_optimization()
            // warn that has happened, suggest --ignore
            return println!("i want an egg");
        }

        if let Some(ImgSubcommands::Gen { .. }) = self.subcommands {
            return println!("Handle_gen({:?})", self.subcommands);
        }

        if let Some(ImgSubcommands::Get { .. }) = self.subcommands {
            return println!("Handle_get({:?})", self.subcommands);
        }

        // handle_optimization(path);
    }
}
