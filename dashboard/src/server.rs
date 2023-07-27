use serde::{Deserialize, Serialize};

/// A representation of the server we are monitoring
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Server {
    #[serde(skip)]
    pub endpoint: String,
    pub hostname: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_space: u64,
    pub available_space: u64,
    pub cpu_amount: usize,
    pub cpu_usage: f32,
}
