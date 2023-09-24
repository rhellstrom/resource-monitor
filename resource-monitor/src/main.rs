mod resources;
mod args;

use crate::resources::{Resources};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use axum::http::header::CONTENT_TYPE;
use clap::Parser;
use tokio::time::sleep;
use args::Args;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    //Initialize resources and wrap in an Arc and Mutex to be shared across threads
    let resource_mutex = Arc::new(Mutex::new(Resources::new()));
    tokio::spawn(refresh_loop(Arc::clone(&resource_mutex), args.update_frequency));

    let app = Router::new().route("/resources", get(move || {
        //Clone arc to increase reference count
        let resource_mutex = Arc::clone(&resource_mutex);
        async move {
            let resource = resource_mutex.lock().unwrap();
            let json_response = resource.serialize(); //Content-Type JSON
            axum::http::Response::builder()
                .header(CONTENT_TYPE, "application/json")
                .body(json_response)
                .unwrap()
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
        let mut resource = resources.lock().unwrap();
        resource.refresh();
    }
}