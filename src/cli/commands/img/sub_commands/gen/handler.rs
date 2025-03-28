use reqwest::blocking::Client;
use serde_json::json;

use super::args::Gen;
use crate::{
    cli::commands::img::{sub_commands::gen::manager::GenManager, ImgError},
    constants::{KEYRING_OPEN_API_KEY, KEYRING_SERVICE},
    error::Result,
    services::{ai, img::Img, keychain::keychain},
    state::AppContext,
};

impl Gen {
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        let mut manager = GenManager::new(ctx, &self);
        manager.handle_args();

        let api_key = keychain(KEYRING_SERVICE, KEYRING_OPEN_API_KEY)?;

        let url = ai::sync::image(&api_key, "a horse")
            .generate()
            .expect("image failed to generate");

        println!("{url}");
        // Img::download(&url);

        Ok(())
    }
}
