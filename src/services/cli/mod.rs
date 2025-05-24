use std::cell::OnceCell;

use crate::{
    constants::KEYRING_SERVICE,
    services::{
        clipboard::ClipboardService,
        editor_launcher::EditorLauncherService,
        keyring::KeyringService,
        prompt::{PasswordDisplay, PromptService},
    },
    utils::Keyring,
    CliConfig,
};

use super::{ai::service::AiService, keyring::KeyringError};

pub mod traits;

pub struct CliService {
    password_input_display_mode: PasswordDisplay,
    pub prompt: OnceCell<PromptService>,
    pub clipboard: OnceCell<ClipboardService>,
    pub keyring: OnceCell<KeyringService>,
    pub ai: OnceCell<AiService>,
    pub editor_launcher: OnceCell<EditorLauncherService>,
}

impl CliService {
    pub fn new(config: &CliConfig) -> Self {
        Self {
            password_input_display_mode: config.password_input_display_mode,
            prompt: OnceCell::new(),
            clipboard: OnceCell::new(),
            keyring: OnceCell::new(),
            ai: OnceCell::new(),
            editor_launcher: OnceCell::new(),
        }
    }

    pub fn init_ai(&self) -> Result<&AiService, KeyringError> {
        let keyring_service = self.init_keyring();
        let api_key = keyring_service.get_or_set(Keyring::OpenAiKey)?;
        let service = self.ai.get_or_init(|| AiService::new(&api_key));
        Ok(service)
    }

    pub fn init_keyring(&self) -> &KeyringService {
        self.keyring.get_or_init(|| {
            let mut service = KeyringService::new(KEYRING_SERVICE);
            service.set_password_mode(&self.password_input_display_mode.into());
            service
        })
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
