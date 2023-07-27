mod resources;

use resources::Resources;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "http://localhost:3000/resources"; // Replace with the appropriate port and path if needed

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    
    let status = response.status();

    println!("Status: {}", status);

    // If the response is successful, print the body
    if response.status().is_success() {
        let body = response.text().await?;
        // Deserialize the JSON response into the Resources struct
        println!("{}\n\n\n", body);


        let resources: Resources = serde_json::from_str(&body).unwrap_or_else(|e| {
            println!("Error while deserializing: {}", e);
            Resources{
                hostname: "".to_string(),
                total_memory: 0,
                used_memory: 0,
                total_space: 0,
                available_space: 0,
                cpu_amount: 0,
                cpu_usage: 0.0,
            } // Return a default instance of Resources in case of error
        });

        // Now you can access the fields of the Resources struct
        println!("{:?}", resources);
    }
    Ok(())
}
