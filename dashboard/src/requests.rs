use std::sync::{Arc};
use std::time::Duration;
use tokio::time::sleep;
use crate::server::Server;
use reqwest::{Client, StatusCode};
use tokio::sync::Mutex;

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

/// To be spawned in a thread. Sleeps update_frequency amount of seconds,
/// acquires the mutex and calls get_servers() on the vector in a loop
pub async fn refresh_servers(servers: Arc<Mutex<Vec<Server>>>, client: Client, update_frequency: u64){
    loop {
        sleep(Duration::from_secs(update_frequency)).await;
        let mut servers = servers.lock().await;
        get_servers(&mut servers, &client).await;
    }
}