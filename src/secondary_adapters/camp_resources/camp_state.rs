use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::secondary_adapters::save::FileSystem;

pub static CAMP_STATE: Lazy<CampState> = 
    Lazy::new(
        || match FileSystem::get_saved_state::<CampState>("camp".to_string()) {
            Ok(saved_state) => return saved_state,
            Err(e) => {
                eprintln!("could not get the {} state: {}", "camp", e);
                return CampState::new();
            }
        },
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
