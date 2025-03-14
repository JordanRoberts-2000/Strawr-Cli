#[derive(clap::Parser, Debug)]
#[command(arg_required_else_help = true)]
pub struct Gen {
    #[arg(help = "Path to img file or folder containing images", short, long)]
    pub description: String,
}
