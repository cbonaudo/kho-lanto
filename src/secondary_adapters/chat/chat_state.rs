use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::domain::MessageHandle;

pub static CHAT_STATE: Lazy<ChatState> = Lazy::new(|| ChatState::new());

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
