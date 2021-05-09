use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::{domain::MessageHandle, secondary_adapters::save::FileSystem};

pub static CHAT_STATE: Lazy<ChatState> = 
    Lazy::new(
        || match FileSystem::get_saved_state::<ChatState>("chat".to_string()) {
            Ok(saved_state) => return saved_state,
            Err(e) => {
                eprintln!("could not get the {} state: {}", "chat", e);
                return ChatState::new();
            }
        },
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
