use gen::args::Gen;
use get::args::Get;

pub mod gen;
pub mod get;

#[derive(clap::Subcommand, Debug)]
pub enum ImgSubcommands {
    Get(Get),
    Gen(Gen),
}
