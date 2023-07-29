mod server;
mod requests;

use std::sync::{Arc};
use std::thread::sleep;
use std::time::Duration;
use reqwest::Error;
use crate::requests::{refresh_servers};
use crate::server::init_with_endpoint;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut server_endpoints: Vec<String> = vec![];
    let url = "http://localhost:3000/resources".to_string();
    let url2 = "faulty.endpoint:3000/resources".to_string();
    server_endpoints.push(url);
    server_endpoints.push(url2);

    let servers = Arc::new(Mutex::new(init_with_endpoint(server_endpoints)));
    println!("{:?}", servers);
    println!();
    println!();


    let servers_clone = Arc::clone(&servers);
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(refresh_servers(servers_clone, 4))
    });

    loop {
        sleep(Duration::from_secs(4));
        println!("{:?}", servers.lock().await);
        println!();
    }
}
