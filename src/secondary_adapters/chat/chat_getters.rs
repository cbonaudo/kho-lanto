use once_cell::sync::Lazy;

use crate::domain::MessageHandle;

use super::chat_state::{ChatState, CHAT_STATE};

pub struct ChatGetters;

impl ChatGetters {
    pub fn get_state_as_string() -> String {
        let chat_state: &ChatState = Lazy::<ChatState>::force(&CHAT_STATE);
        serde_json::to_string(chat_state).unwrap()
    }

    pub fn get_handle(handle: String) -> Option<MessageHandle> {
        let message_handle_list = CHAT_STATE.message_handle_list.lock().unwrap();
        message_handle_list
            .iter()
            .position(|message_handle| message_handle.handle == handle)
            .map(|index| message_handle_list[index].clone())        
    }
}
