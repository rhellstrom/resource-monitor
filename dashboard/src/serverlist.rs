use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::time::{interval, sleep};
use crate::server::Server;
use reqwest::{Client, StatusCode};
use tokio::sync::Mutex;

//Servers a mutex?
pub struct Serverlist {
    pub servers: Arc<Mutex<Vec<Server>>>,
    endpoints: Vec<String>,
    client: Client,
    exit_refresh: Arc<AtomicBool>,
    update_interval: u64,
}

//TODO: Clean up the cloning of endpoints
impl Serverlist {
    pub fn new(endpoints: Vec<String>) -> Serverlist {
        Serverlist {
            servers: Arc::new(Mutex::new(init_with_endpoint(&endpoints))),
            endpoints,
            client: Client::new(),
            exit_refresh: Arc::new(AtomicBool::new(false)),
            update_interval: 2000,
        }
    }
    async fn fetch_data(&mut self) -> Vec<Server>{
        let mut temp_container = init_with_endpoint(&self.endpoints);

        for server in temp_container.iter_mut() {
            let endpoint = server.endpoint.clone();
            if let Ok(response) = self.client.get(&endpoint).send().await {
                if response.status() == StatusCode::OK {
                    let body = response.text().await.unwrap();
                    *server = serde_json::from_str(&body).unwrap();
                    server.endpoint = endpoint;
                }
            }
        }
        temp_container
    }

    //Create a servers clone and a reference clone?
    pub async fn refresh_list(&mut self)  {
        let exit_refresh = Arc::clone(&self.exit_refresh);
        let servers_mutex = self.servers.clone(); // Create a local reference

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(self.update_interval));
            while !exit_refresh.load(Ordering::Relaxed) {
                let updated_data = self.fetch_data().await;
                let mut servers= servers_mutex.lock().await;
                *servers = updated_data;
                interval.tick().await;
            }
        });
    }

    pub fn stop_refresh(&mut self){
        self.exit_refresh.store(true, Ordering::Relaxed);
    }
}

pub fn init_with_endpoint(endpoints: &Vec<String>) -> Vec<Server> {
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
