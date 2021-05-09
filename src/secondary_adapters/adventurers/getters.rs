use once_cell::sync::Lazy;

use super::state::{State, ADVENTURERS_STATE};

pub struct Getters;

impl Getters {
    pub fn get_state_as_string() -> String {
        let adventurers_state: &State = Lazy::<State>::force(&ADVENTURERS_STATE);
        serde_json::to_string(adventurers_state).unwrap()
    }
}
