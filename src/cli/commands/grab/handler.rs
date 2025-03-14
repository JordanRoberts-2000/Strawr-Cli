use crate::{error::Result, state::AppContext};

use super::command::args::GrabCommand;

pub struct Grab {}

impl Grab {
    pub fn execute(args: &GrabCommand, ctx: &AppContext) -> Result<()> {
        log::trace!("Command 'grab' called");
        GrabCommand::execute(args, ctx)?;
        log::trace!("command 'grab' executed successfully");
        Ok(())
    }
}
