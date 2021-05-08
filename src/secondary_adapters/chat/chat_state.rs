use once_cell::sync::Lazy;

use crate::domain::MessageHandle;

pub static CHAT_STATE: Lazy<ChatState> = Lazy::new(|| ChatState::new());

pub struct ChatState {
    pub wood: Option<String>,
    pub fire: Option<String>,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            wood: None,
            fire: None,
        }
    }

    pub fn save_message_handle(message_handle: MessageHandle) {}
}
