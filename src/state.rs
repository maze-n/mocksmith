use serde_json::Value;
use std::{collections::HashMap, path::PathBuf};

pub struct AppState {
    pub path: PathBuf,
}

impl AppState {
    pub fn new(path: PathBuf) -> Self {
        AppState { path }
    }

    pub fn get_data(&mut self) -> HashMap<String, Value> {
        let results: Result<String, std::io::Error> = std::fs::read_to_string(&self.path);

        match results {
            Err(e) => panic!("Failed to read from file: {}", e),
            Ok(content) => {
                let parsed: HashMap<String, Value> =
                    serde_json::from_str(content.as_str()).expect("Unable to parse JSON file!");

                return parsed;
            }
        }
    }
}
