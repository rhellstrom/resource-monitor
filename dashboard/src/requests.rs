use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time::sleep;
use crate::server::{init_with_endpoint, Server};
use reqwest::{Client, StatusCode};
use tokio::sync::Mutex;


/// Iterates through the vector of Server and makes a GET request to each endpoint
/// and updates the struct if we got a status code 200 in the response.
/// Otherwise we silently fail and try again next iteration
pub async fn get_servers(servers: &mut [Server], client: &Client){
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

/// Creates a Client to make requests with, updates a vector of Servers and once done,
/// we lock the mutex and update the data
pub async fn refresh_servers(servers: Arc<Mutex<Vec<Server>>>, update_frequency: u64,
                             exit_loop: Arc<AtomicBool>, endpoints: Vec<String>){
    let client = Client::new();
    let mut servers_container = init_with_endpoint(endpoints);
    while !exit_loop.load(Ordering::Relaxed) {
        sleep(Duration::from_secs(update_frequency)).await;
        get_servers(&mut servers_container, &client).await;

        //Only lock and update mutex after we've fetched data
        let mut servers_data = servers.lock().await;
        *servers_data = servers_container.clone();
    }
}
