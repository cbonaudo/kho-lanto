use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::{domain::dto::Adventurer, secondary_adapters::save::FileSystem};

pub static ADVENTURERS_STATE: Lazy<State> =
    Lazy::new(
        || match FileSystem::get_saved_state::<State>("adventurers".to_string()) {
            Ok(saved_state) => return saved_state,
            Err(e) => {
                eprintln!("could not get the {} state: {}", "adventurers", e);
                return State::new();
            }
        },
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
