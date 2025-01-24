use args::PlaygroundCommand;

use crate::state::AppContext;

pub mod args;

impl PlaygroundCommand {
    pub fn handle_command(&self, ctx: &AppContext) {
        ctx.debug_log("Playground Command Called");
    }
}
