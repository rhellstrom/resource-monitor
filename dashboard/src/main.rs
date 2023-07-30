mod server;
mod requests;
mod terminal;

use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use anyhow::{Context, Result};
use crate::requests::{refresh_servers};
use crate::server::init_with_endpoint;
use tokio::sync::Mutex;
use crate::terminal::{restore_terminal, run, setup_terminal};

#[tokio::main]
async fn main() -> Result<()> {
    let mut server_endpoints: Vec<String> = vec![];
    let url = "http://localhost:3000/resources".to_string();
    let url2 = "faulty.endpoint:3000/resources".to_string();
    server_endpoints.push(url);
    server_endpoints.push(url2);

    let servers = Arc::new(Mutex::new(init_with_endpoint(server_endpoints)));
    println!("{:?}", servers);
    println!();
    println!();

    // Create an atomic bool wrapped in an Arc to pass to the refresh_thread
    let exit_loop = Arc::new(AtomicBool::new(false));
    let servers_clone = Arc::clone(&servers);
    let exit_loop_clone = exit_loop.clone(); //Clone to share reference

    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(refresh_servers(servers_clone, 4, exit_loop_clone))
    });


    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;

    //Shut down the refresh thread by altering the AtomicBool value
    exit_loop.store(true, Ordering::Relaxed);

    Ok(())
}
