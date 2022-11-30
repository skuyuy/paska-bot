pub mod release;

use std::{
    collections::HashMap,
    fs
};
use uuid::Uuid;
use release::Release;

#[derive(Debug)]
pub struct ReleaseStore {
    store: HashMap<Uuid, Release>
}

impl ReleaseStore {
    pub fn new() -> ReleaseStore {
        ReleaseStore { store: HashMap::new() }
    }

    /// Reads the release table from a json
    pub fn read(&mut self, path: &str) -> Result<(), String> {
        match fs::read_to_string(path) {
            Ok(v) => match serde_json::from_str(v.as_str()) {
                Ok(deserialized_val) => Ok(self.store = deserialized_val),
                Err(err) => Err(format!("Could not deserialize: {}", err))
            },
            Err(err) => Err(format!("Could not read file: {}", err))
        }        
    }

    /// writes the release table to a json
    pub fn write(&self, path: &str) -> Result<(), String> {
        let serialize_result = serde_json::to_string(&self.store);
        match serialize_result {
            Ok(contents) => match fs::write(path, contents) {
                Ok(_) => Ok(()),
                Err(err) => Err(format!("Could not write file: {}", err))
            },
            Err(err) => Err(format!("Could not serialize: {}", err))
        }
    }
}