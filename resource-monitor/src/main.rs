mod resources;

use crate::resources::{Resources};
use axum::{routing::get, Router, Json};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let resource_mutex = Arc::new(Mutex::new(Resources::new()));
    // Spawn a thread that will refresh the Resources struct
    tokio::spawn(refresh_loop(Arc::clone(&resource_mutex)));

    // Clone the Arc to increase reference count, allowing multiple threads to share ownership
    let app = Router::new().route("/", get(move || {
        let resource_mutex = Arc::clone(&resource_mutex);
        async move {
            let resource = resource_mutex.lock().unwrap();
            Json(resource.serialize()) //Content-Type JSON
        }
    }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn refresh_loop(resources: Arc<Mutex<Resources>>) {
    loop {
        // Sleep for 2000 ms
        sleep(Duration::from_millis(2000)).await;

        // Refresh the Resources struct
        let mut resource = resources.lock().unwrap();
        resource.refresh();
    }
}