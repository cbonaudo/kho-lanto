use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

use crate::domain::MessageHandle;

pub static CHAT_STATE: Lazy<ChatState> = Lazy::new(||
        // TODO: refactor this
        if let Ok(file_contents) = fs::read("./state_saves/chat") {
            if let Ok(saved_state) = String::from_utf8(file_contents) {
                return serde_json::from_str(&saved_state).unwrap()
            } else {
                return ChatState::new()
            }
        } else {
            return ChatState::new()
        }
    );

#[derive(Deserialize, Serialize)]
pub struct ChatState {
    pub message_handle_list: Mutex<Vec<MessageHandle>>,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            message_handle_list: Mutex::new(vec![]),
        }
    }
}
