use std::io;

use crate::{commands::uninstall::UninstallCommand, state::AppContext};

impl UninstallCommand {
    pub fn confirm_uninstallation(&self, ctx: &AppContext) -> bool {
        println!(
            "Are you sure you want to uninstall and delete all configuration data at '{}'? (y/N)",
            ctx.config_path.display()
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        ctx.debug_log(&format!("input: {}", input.as_str()));
        ctx.debug_log(&format!(
            "transformed input: {}",
            input.trim().to_lowercase().as_str()
        ));

        matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
    }
}
