use crate::{error::Result, state::AppContext};

use super::{args::Get, manager::GetManager};

impl Get {
    pub fn execute(&self, ctx: &AppContext) -> Result<()> {
        let mut manager = GetManager::new(&self.path, &ctx)?;

        if self.color {
            manager.handle_color()?;
        } else if self.data_url {
            manager.handle_data_url()?;
        } else if self.blur_data_url {
            manager.handle_blur_url()?;
        } else if self.alt {
            manager.handle_alt()?;
        } else if self.hash {
            manager.handle_blurhash()?;
        } else {
            manager.handle_default()?;
        }

        Ok(())
    }
}
