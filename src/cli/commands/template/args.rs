use clap::{Args, Parser};

#[derive(Parser, Debug)]
pub struct TemplateCommand {
    #[arg()]
    pub template: Option<String>,

    #[arg(long, short)]
    pub variant: Option<String>,

    #[arg(long, short)]
    pub backend: Option<String>,

    #[arg(long, short)]
    pub frontend: Option<String>,
}
