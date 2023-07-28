mod server;
mod requests;

use server::Server;
use reqwest::Error;
use crate::requests::get_resources;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut server_endpoints: Vec<String> = vec![];
    let url = "http://localhost:3000/resources".to_string();
    let url2 = "faulty.endpoint:3000/resources".to_string();
    server_endpoints.push(url);
    server_endpoints.push(url2);

    let mut servers: Vec<Server> = vec![];

    //Create a Server for each endpoint we wish to track and populate with default values until we call refresh
    for endpoint in server_endpoints {
        servers.push(Server {
            endpoint,
            ..Default::default()
        })
    }
    println!("{:?}", servers);
    println!();
    println!();
    let client = reqwest::Client::new();

    get_resources(&mut servers, client).await;

    println!("{:?}", servers);
    Ok(())
}
