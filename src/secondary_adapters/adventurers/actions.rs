use crate::domain::dto::{Adventurer, Occupation};

use super::state::ADVENTURERS_STATE;

pub struct Actions;

impl Actions {
    pub fn assign(adventurer_name: String, occupation: Occupation) {
        let mut adventurers_list = ADVENTURERS_STATE.adventurers_list.lock().unwrap();

        match adventurers_list
            .iter()
            .position(|adventurer| adventurer.name == adventurer_name)
        {
            Some(i) => {
                adventurers_list[i].occupation = occupation;
            }
            None => adventurers_list.push(Adventurer {
                name: adventurer_name,
                occupation,
            }),
        }
    }
}
