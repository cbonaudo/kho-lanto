use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs, sync::Mutex};

use crate::domain::dto::Adventurer;

pub static ADVENTURERS_STATE: Lazy<State> = Lazy::new(||
    // TODO: refactor this
    if let Ok(file_contents) = fs::read("./state_saves/adventurers") {
        if let Ok(saved_state) = String::from_utf8(file_contents) {
            if let Ok(state_parsed) = serde_json::from_str(&saved_state) {
                return state_parsed
            } else {
                return State::new()
            }
        } else {
            return State::new()
        }
    } else {
        return State::new()
    }
);

#[derive(Deserialize, Serialize)]
pub struct State {
    pub adventurers_list: Mutex<Vec<Adventurer>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            adventurers_list: Mutex::new(vec![]),
        }
    }
}
