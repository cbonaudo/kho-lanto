use once_cell::sync::Lazy;

use super::camp_state::{CampState, CAMP_STATE};

pub struct CampGetters;

impl CampGetters {
    pub fn get_wood_amount() -> u128 {
        CAMP_STATE.wood
    }

    pub fn is_fire_tryable() -> bool {
        CAMP_STATE.wood > 9
    }

    pub fn get_state_as_string() -> String {
        let camp_state: &CampState = Lazy::<CampState>::force(&CAMP_STATE);
        serde_json::to_string(camp_state).unwrap()
    }
}
