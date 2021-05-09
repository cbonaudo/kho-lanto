use std::fs;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CAMP_STATE: Lazy<CampState> = Lazy::new(||
    // TODO: refactor this
    if let Ok(file_contents) = fs::read("./state_saves/camp") {
        if let Ok(saved_state) = String::from_utf8(file_contents) {
            if let Ok(state_parsed) = serde_json::from_str(&saved_state) {
                return state_parsed
            } else {
                return CampState::new()
            }
        } else {
            return CampState::new()
        }
    } else {
        return CampState::new()
    }
);

#[derive(Deserialize, Serialize)]
pub struct CampState {
    pub wood: u128,
    pub fire_tries: u8,
    pub fire: bool,
}

impl CampState {
    pub fn new() -> Self {
        Self {
            wood: 0,
            fire_tries: 0,
            fire: false,
        }
    }
}
