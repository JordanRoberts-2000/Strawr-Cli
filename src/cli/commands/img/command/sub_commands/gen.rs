#[derive(clap::Parser, Debug, Clone)]
#[command(arg_required_else_help = true, group(
    clap::ArgGroup::new("orientation")
        .args(&["wide", "tall", "size"])
        .multiple(false)
))]
pub struct GenSubcommand {
    #[arg(help = "Text description or prompt for image generation")]
    pub description: String,

    #[arg(
        short,
        long,
        action = clap::ArgAction::SetTrue,
        help = "Generate a widescreen image (horizontal orientation)"
    )]
    pub wide: Option<bool>,

    #[arg(
        short,
        long,
        action = clap::ArgAction::SetTrue,
        help = "Generate a tall image (vertical orientation)"
    )]
    pub tall: Option<bool>,

    #[arg(
        short,
        long,
        help = "Specify the DALL-E version to use (must be either '2' or '3')",
        value_parser = clap::value_parser!(u8).range(2..=3)
    )]
    pub dalle: Option<u8>,

    #[arg(
        short,
        long,
        help = "Specify a custom image size (overrides tall/wide options)"
    )]
    pub size: Option<ImageSize>,
}
