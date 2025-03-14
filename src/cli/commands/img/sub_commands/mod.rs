use gen::args::Gen;

pub mod gen;
pub mod get;

#[derive(clap::Subcommand, Debug)]
pub enum ImgSubcommands {
    Get,
    Gen(Gen),
}
