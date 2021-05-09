use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Occupation {
    WOOD,
    FIRE,
    EXPLORATION,
    CHILLING,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Adventurer {
    pub name: String,
    pub occupation: Occupation,
}
