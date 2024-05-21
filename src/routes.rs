use crate::state::AppState;

use rocket::{get, State};

use std::path::PathBuf;
use std::sync::Mutex;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<entity_key>")]
pub fn get_entity(entity_key: PathBuf, _state: &State<Mutex<AppState>>) -> String {
    let entity_key = entity_key.to_str().unwrap();
    // let data: HashMap<String, Value> = state.inner().lock().unwrap().get_data();

    // if !data.contains_key(entity_key) {
    //     // Status::NotFound
    // }

    return entity_key.to_string();
}
