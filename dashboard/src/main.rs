mod server;
mod terminal;
mod ui;
mod app;
mod util;
mod args;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use anyhow::{Result};
use clap::Parser;
use tokio::sync::Mutex;
use crate::args::Args;
use crate::server::{refresh_servers};
use crate::server::init_with_endpoint;
use crate::terminal::{run};
use crate::util::extract_endpoints_from_files;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let server_endpoints = extract_endpoints_from_files(args.files);

    let servers = Arc::new(Mutex::new(init_with_endpoint(server_endpoints.clone())));
    // Create an atomic bool wrapped in an Arc to pass to the refresh_thread
    let exit_loop = Arc::new(AtomicBool::new(false));
    let servers_clone = Arc::clone(&servers);
    let exit_loop_clone = exit_loop.clone(); //Clone to share reference

    tokio::spawn(async move {
        refresh_servers(servers_clone, args.update_frequency, exit_loop_clone, server_endpoints).await;
    });
    run(Arc::clone(&servers), args.tick_rate, args.update_frequency).await.expect("Application loop failure");
    //Shut down the refresh thread by altering the AtomicBool value
    exit_loop.store(true, Ordering::Relaxed);
    Ok(())
}