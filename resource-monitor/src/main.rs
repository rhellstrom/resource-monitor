mod resources;
mod args;

use crate::resources::{Resources};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::{Arc};
use std::time::Duration;
use axum::http::header::CONTENT_TYPE;
use clap::Parser;
use tokio::sync::Mutex;
use tokio::time::sleep;
use args::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let server_resources = Arc::new(Mutex::new(Resources::new()));

    tokio::spawn(refresh_loop(Arc::clone(&server_resources), args.update_frequency));

    let app = Router::new().route("/resources", get(move || {
        let resource_mutex = Arc::clone(&server_resources);
        async move {
            let resource = resource_mutex.lock().await;
            match resource.serialize() {
                Ok(json_response) => {
                    axum::http::Response::builder()
                        .header(CONTENT_TYPE, "application/json")
                        .body(json_response)
                        .unwrap()
                }
                Err(err) => {
                    eprintln!("Error serializing resource: {:?}", err);
                    axum::http::Response::builder()
                        .status(500)
                        .body("Internal Server Error".to_string())
                        .unwrap()
                }
            }
        }
    }));

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn refresh_loop(resources: Arc<Mutex<Resources>>, update_frequency: u64) {
    loop {
        sleep(Duration::from_millis(update_frequency)).await;
        let mut resource = resources.lock().await;
        resource.refresh();
    }
}