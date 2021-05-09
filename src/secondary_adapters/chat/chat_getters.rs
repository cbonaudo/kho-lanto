use once_cell::sync::Lazy;

use super::chat_state::{CHAT_STATE, ChatState};

pub struct ChatGetters;

impl ChatGetters {
    pub fn get_state_as_string() -> String {
        let chat_state: &ChatState = Lazy::<ChatState>::force(&CHAT_STATE);
        serde_json::to_string(chat_state).unwrap()
    }
}
