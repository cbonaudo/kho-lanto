use std::fs;

pub struct FileSystem;

impl FileSystem {
    pub fn save_state(file_name: String, state: String) {
        fs::write(format!("./state_saves/{}", file_name), state).unwrap();
    }
}
