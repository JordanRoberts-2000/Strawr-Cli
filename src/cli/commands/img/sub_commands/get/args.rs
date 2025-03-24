#[derive(clap::Parser, Debug)]
#[command()]
pub struct Get {
    #[arg(help = "Path to img file or folder (positional argument)")]
    pub path: String,

    #[arg(
        help = "Retrieve the data URL representation of the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub data_url: bool,

    #[arg(
        help = "Retrieve the blur data URL representation of the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub blur_data_url: bool,

    #[arg(
        help = "Retrieve the format of the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub format: bool,

    #[arg(
        help = "Retrieve the dominant color or color profile of the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub color: bool,

    #[arg(
        help = "Generate or retrieve alternative text (alt) for the image",
        short,
        long,
        action = clap::ArgAction::SetTrue
    )]
    pub alt: bool,
}
