use anyhow::{anyhow, Result as AnyResult};
use serde::de::DeserializeOwned;
use std::fs;

pub struct FileSystem;

impl FileSystem {
    pub fn save_state(file_name: String, state: String) {
        fs::write(format!("./state_saves/{}", file_name), state).unwrap();
    }

    pub fn get_saved_state<T>(state_name: String) -> AnyResult<T>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(
            &String::from_utf8(
                fs::read(format!("./state_saves/{}", state_name))
                    .map_err(|err| anyhow!("could not read from state_saves folder: {}", err))?,
            )
            .map_err(|err| anyhow!("could not parse string from bytes: {}", err))?,
        )
        .map_err(|err| anyhow!("could not parse string to State: {}", err))
    }
}
