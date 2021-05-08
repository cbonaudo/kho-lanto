use once_cell::sync::Lazy;

pub static CAMP_STATE : Lazy<CampState> = Lazy::new(|| {
    CampState::new()
});

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