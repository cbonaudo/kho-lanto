use crate::secondary_adapters::{camp::CampGetters, chat::ChatGetters, save::FileSystem};

pub struct SaveFile;

impl SaveFile {
    pub fn save_states() {
        let chat_state = ChatGetters::get_state_as_string();
        FileSystem::save_state("chat".to_string(), chat_state);

        let camp_state = CampGetters::get_state_as_string();
        FileSystem::save_state("camp".to_string(), camp_state)
    }
}
