mod http;
mod routes;
mod state;

use clap::Parser;
use std::ffi::OsStr;
use std::path::PathBuf;

#[macro_use]
extern crate rocket;

use crate::http::Server;

#[derive(Parser)]
struct Cli {
    file_path: PathBuf,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    if !is_file_valid(&args.file_path) {
        return;
    }

    let mut server: Server = Server::new();
    server.start(args.file_path).await;
}

fn is_file_valid(file_path: &PathBuf) -> bool {
    if !file_path.exists() {
        println!("Please pass a valid file path!");
        return false;
    }
    if !file_path.is_file() {
        println!("The path is not a file!");
        return false;
    }

    let result: Option<&str> = (file_path).extension().and_then(OsStr::to_str);

    match result {
        Some(file_extension) => {
            if file_extension != "json" {
                println!("Please pass a valid JSON file!");
                return false;
            } else {
                return true;
            }
        }
        None => {
            println!("Failed to detect file extension!");
            return false;
        }
    }
}
