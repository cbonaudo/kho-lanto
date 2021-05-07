use crate::models::camp_state::CAMP_STATE;

pub struct CampResources;

impl CampResources {
    pub fn get_wood_amount() -> u128 {
        CAMP_STATE.wood
    }

    pub fn is_fire_tryable() -> bool {
        CAMP_STATE.wood > 9
    }
}
