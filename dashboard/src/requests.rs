use crate::server::Server;
use reqwest::{Client, StatusCode};

//Maybe we should init each struct as default and run them all through a refresh function instead?
//Saving up on a function

/// Requests data from each server endpoint and deserializes it into a Server.
/// If an endpoint returns a status other than 200 we add a Server with default values instead.
/// Returns a vector with each element representing a server.
pub async fn get_resource(endpoints: Vec<String>, client: Client) -> Vec<Server> {
    let mut servers: Vec<Server> = vec![];

    for url in endpoints{
        let response = client.get(&url).send().await.unwrap();
        match response.status() {
            StatusCode::OK => {
                let body = response.text().await.unwrap();
                let mut server_resources: Server = serde_json::from_str(&body).unwrap();
                server_resources.endpoint = url;
                servers.push(server_resources);
            }
            _ => {
                servers.push(Server {
                    endpoint: url,
                    ..Default::default()
                });
            }
        }
    }
    servers
}

/*
//Acquire lock, fetch resource and update field of struct for each server
pub async fn refresh_resources(servers: Vec<Server>, interval: u64, client: Client){

} */