use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time::sleep;
use crate::server::Server;
use reqwest::{Client, StatusCode};
use tokio::sync::Mutex;

pub struct Serverlist {
    pub servers: Vec<Server>,
    client: Client,
    exit_refresh: Arc<AtomicBool>,
    update_interval: u64,
}

impl Serverlist {
    pub fn new(endpoints: Vec<String>) -> Serverlist {
        Serverlist {
            servers: init_with_endpoint(endpoints),
            client: Client::new(),
            exit_refresh: Arc::new(AtomicBool::new(false)),
            update_interval: 2000,
        }
    }
    async fn fetch_data(&mut self){
        for server in self.servers.iter_mut() {
            let endpoint = server.endpoint.clone();
            if let Ok(response) = self.client.get(&endpoint).send().await {
                if response.status() == StatusCode::OK {
                    let body = response.text().await.unwrap();
                    *server = serde_json::from_str(&body).unwrap();
                    server.endpoint = endpoint;
                }
            }
        }
    }
    pub fn refresh_list(&self){
        let exit_refresh = Arc::clone(&self.exit_refresh);
        let servers_clone = self.servers.clone();
        //let update_interval = self.update_interval;
    }

    pub fn stop_refresh(&mut self){
        self.exit_refresh = true;
    }
}

pub fn init_with_endpoint(endpoints: Vec<String>) -> Vec<Server> {
    let mut servers: Vec<Server> = vec![];
    for endpoint in endpoints {
        servers.push(Server {
            endpoint,
            ..Default::default()
        })
    }
    servers
}

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
        let mut servers = servers.lock().await;
        get_servers(&mut servers, &client).await;
        //This holds the mutexguard in scope..
        sleep(Duration::from_secs(update_frequency)).await;
    }
}
