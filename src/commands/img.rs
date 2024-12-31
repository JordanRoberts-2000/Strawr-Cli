use crate::args::{Commands, ImgSubcommands};

pub fn handle_commands_img(commands: &Commands) {
    if let Commands::Img {
        path,
        subcommands,
        output,
        format,
        size,
        crop,
        ignore,
    } = commands
    {
        if subcommands.is_some() && !ignore {
            // check if valid folder or file exists in cwd
            // if yes handle_optimization()
            // warn that has happened, suggest --ignore
            return println!("i want an egg");
        }

        if let Some(ImgSubcommands::Gen { .. }) = subcommands {
            return println!("Handle_gen({:?})", subcommands);
        }

        if let Some(ImgSubcommands::Get { .. }) = subcommands {
            return println!("Handle_get({:?})", subcommands);
        }

        // handle_optimization(path);
    }
}
