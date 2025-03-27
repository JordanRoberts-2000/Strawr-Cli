use super::GenManager;

impl GenManager {
    pub fn handle_args(&mut self) {
        if let Some(version) = self.args.dalle {
            self.dalle_version = version;
        }

        if let Some(size) = &self.args.size {
            self.size = size.resolution();
        } else if self.args.wide.unwrap_or(false) {
            self.dalle_version = 3;
            self.size = "1792x1024";
        } else if self.args.tall.unwrap_or(false) {
            self.dalle_version = 3;
            self.size = "1024x1792";
        }
    }
}
