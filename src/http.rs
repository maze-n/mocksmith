use rocket::get;
use std::path::PathBuf;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct Server {
    file_path: PathBuf,
}

impl Server {
    pub fn new(file_path: PathBuf) -> Server {
        Server { file_path }
    }

    pub async fn start(&self) {
        let server = rocket::build().mount("/", routes![index]);

        let result = server.launch().await;
        match result {
            Err(_) => {
                println!("Error: Failed to start the server!");
            }
            Ok(_) => {}
        }
    }
}
