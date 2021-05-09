use crate::secondary_adapters::chat::ChatGetters;

pub struct SaveFile;

impl SaveFile {
    pub fn save_states() {
        let chat_state = ChatGetters::get_state_as_string();
        dbg!(chat_state);
    }
}