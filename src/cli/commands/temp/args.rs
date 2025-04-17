use clap::Parser;

use crate::utils::Editor;

#[derive(Parser, Debug)]
pub struct TempCommand {
    #[arg(short, long, value_enum, ignore_case = true)]
    pub editor: Option<Editor>,
}
