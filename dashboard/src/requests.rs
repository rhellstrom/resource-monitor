use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time::sleep;
use crate::server::Server;
use reqwest::{Client, StatusCode};


/// Iterates through the vector of Server and makes a GET request to each endpoint
/// and updates the struct if we got a status code 200 in the response.
/// Otherwise we silently fail and try again next iteration
pub async fn get_servers(servers: &mut [Server], client: &Client) {
    for server in servers.iter_mut() {
        let endpoint = server.endpoint.clone();
        if let Ok(response) = client.get(&endpoint).send().await {
            if response.status() == StatusCode::OK {
                let body = response.text().await.unwrap();
                *server = serde_json::from_str(&body).unwrap();
                server.endpoint = endpoint;
            }
        }
    }
}

/// Creates a reqwest::Client, attempts to acquire the Mutex and calls get_servers()
/// to refresh our structs and then sleeps update_frequency amount of seconds
pub async fn refresh_servers(servers: Arc<Mutex<Vec<Server>>>, update_frequency: u64, exit_loop: Arc<AtomicBool>){
    let client = Client::new();
    loop {
        if exit_loop.load(Ordering::Relaxed) {
            break;
        }

        sleep(Duration::from_secs(update_frequency)).await;
        let mut servers = servers.lock().unwrap();
        get_servers(&mut servers, &client).await;
    }
}