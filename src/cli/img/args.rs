use clap::Subcommand;

#[derive(clap::Parser, Debug)]
pub struct ImgCommands {
    #[command(subcommand)]
    pub subcommands: Option<ImgSubcommands>,

    pub path: Option<String>,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long)]
    pub format: Option<String>,

    #[arg(short, long)]
    pub size: Option<String>,

    #[arg(short, long)]
    pub crop: Option<String>,

    #[arg(short, long)]
    pub ignore: bool,
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
