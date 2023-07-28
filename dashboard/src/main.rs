mod server;
mod requests;

use reqwest::Error;
use crate::requests::get_resources;
use crate::server::init_with_endpoint;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut server_endpoints: Vec<String> = vec![];
    let url = "http://localhost:3000/resources".to_string();
    let url2 = "faulty.endpoint:3000/resources".to_string();
    server_endpoints.push(url);
    server_endpoints.push(url2);

    let mut servers = init_with_endpoint(server_endpoints);
    println!("{:?}", servers);
    println!();
    println!();
    let client = reqwest::Client::new();

    get_resources(&mut servers, client).await;

    println!("{:?}", servers);
    Ok(())
}
