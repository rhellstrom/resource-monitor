use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Server {
    #[serde(skip)]
    pub endpoint: String,
    pub hostname: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_space: u64,
    pub available_space: u64,
    pub cpu_amount: usize,
    pub cpu_usage: f32,
    pub cpu_load_per_core: Vec<f32>,
    pub disk_names: Vec<String>,
    pub disk_available: Vec<u64>,
    pub disk_total: Vec<u64>,
    pub uptime: u64,
    pub os_version: String,
    pub kernel_version: String,
    pub load_avg_one: f64,
    pub load_avg_five: f64,
    pub load_avg_fifteen: f64,
    pub bytes_received: u64,
    pub bytes_transmitted: u64,
}

impl Server{
    pub fn new(endpoint: String) -> Server {
        Server {
            endpoint,
            ..Default::default()
        }
    }
}

/// Initialises a Vec<Server> with default values and an endpoint for each instance of Server
pub fn init_with_endpoint(endpoints: Vec<String>) -> Vec<Server> {
    let mut servers: Vec<Server> = vec![];
    for endpoint in endpoints {
        servers.push(Server::new(endpoint))
    }
    servers
}

/// Iterates through the vector of Server and makes a GET request to each endpoint
/// and updates the struct if we got a status code 200 in the response.
/// Otherwise we silently fail
async fn get_servers(servers: &mut [Server], client: &Client){
    for server in servers.iter_mut() {
        let endpoint = server.endpoint.clone();
        if let Ok(response) = client.get(&endpoint)
            .timeout(Duration::from_secs(2))
            .send().await {
            if response.status() == StatusCode::OK {
                let body = response.text().await.unwrap();
                if let Ok(deserialized_server) = serde_json::from_str(&body) {
                    *server = deserialized_server;
                    server.endpoint = endpoint;
                }
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
        sleep(Duration::from_millis(update_frequency)).await;
        get_servers(&mut servers_container, &client).await;

        //Only lock and update mutex after we've fetched data
        let mut servers_data = servers.lock().await;

        //If we've added a new endpoint from our UI thread we need to avoid overwriting it
        if servers_data.len() != servers_container.len(){
            servers_container.push(servers_data.last().unwrap().clone());
        }
        *servers_data = servers_container.clone();
    }
}