mod server;
mod requests;

use server::Server;
use reqwest::Error;
use crate::requests::get_resource;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut server_endpoints: Vec<String> = vec![];
    let url = "http://localhost:3000/resources".to_string();
    server_endpoints.push(url); // Replace with the appropriate port and path if needed

    let client = reqwest::Client::new();

    let servers: Vec<Server> = get_resource(server_endpoints, client).await;

    println!("{:?}", servers);
    Ok(())
}
