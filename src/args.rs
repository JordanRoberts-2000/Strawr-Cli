use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Watcher,
    #[command(arg_required_else_help(true))]
    Img {
        #[command(subcommand)]
        subcommands: Option<ImgSubcommands>,

        path: Option<String>,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(short, long)]
        format: Option<String>,

        #[arg(short, long)]
        size: Option<String>,

        #[arg(short, long)]
        crop: Option<String>,

        #[arg(short, long)]
        ignore: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ImgSubcommands {
    Gen {
        description: Option<String>,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(short, long)]
        aspect: Option<String>,
    },

    Get {
        path: Option<String>,

        #[arg(short, long)]
        alt: Option<String>,

        #[arg(short, long)]
        color: Option<String>,

        #[arg(short = 'h', long)]
        blurhash: Option<String>,

        #[arg(short = 'b', long)]
        blur_data_url: Option<String>,

        #[arg(short = 'd', long)]
        data_url: Option<String>,
    },
}
