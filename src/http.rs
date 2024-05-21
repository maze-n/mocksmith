use crate::routes::{get_entity, index};
use crate::state::AppState;
use std::path::PathBuf;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub async fn start(&mut self, file_path: PathBuf) {
        // self.load_data();
        let state = AppState::new(file_path);

        let server = rocket::build()
            .manage(std::sync::Mutex::new(state))
            .mount("/", routes![index, get_entity]);

        server.launch().await.ok();
    }
}
