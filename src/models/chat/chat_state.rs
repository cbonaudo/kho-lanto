use once_cell::sync::Lazy;

pub static CHAT_STATE : Lazy<ChatState> = Lazy::new(|| {
    ChatState::new()
});

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
}