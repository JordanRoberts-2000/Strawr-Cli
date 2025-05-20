use std::cell::OnceCell;

use crate::{
    services::{
        clipboard::ClipboardService,
        editor_launcher::EditorLauncherService,
        prompt::{PasswordDisplay, PromptService},
    },
    CliConfig,
};

pub mod traits;

pub struct CliService {
    password_input_display_mode: PasswordDisplay,
    pub prompt: OnceCell<PromptService>,
    pub clipboard: OnceCell<ClipboardService>,
    // pub keychain: Box<dyn Keychain>,
    pub editor_launcher: OnceCell<EditorLauncherService>,
}

impl CliService {
    pub fn new(config: &CliConfig) -> Self {
        Self {
            password_input_display_mode: config.password_input_display_mode,
            prompt: OnceCell::new(),
            clipboard: OnceCell::new(),
            editor_launcher: OnceCell::new(),
        }
    }

    pub fn init_prompt(&self) -> &PromptService {
        self.prompt.get_or_init(|| {
            let mut service = PromptService::new();
            service.set_password_mode(&self.password_input_display_mode.into());
            service
        })
    }

    pub fn init_clipboard(&self) -> &ClipboardService {
        self.clipboard.get_or_init(|| ClipboardService::new())
    }

    pub fn init_launch_editor(&self) -> &EditorLauncherService {
        self.editor_launcher
            .get_or_init(|| EditorLauncherService::new())
    }
}
