use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Resources {
    pub hostname: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_space: u64,
    pub available_space: u64,
    pub cpu_amount: usize,
    pub cpu_usage: f32,
}
