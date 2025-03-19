use crate::error::Result;
use crate::state::AppContext;

use super::args::ImgCommand;

impl ImgCommand {
    pub fn execute(&self, _ctx: &AppContext) -> Result<()> {
        Ok(())
    }
}
