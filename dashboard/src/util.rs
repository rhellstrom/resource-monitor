pub fn total_memory(used: u64, total: u64) -> f64 {
    (used as f64 / total as f64) * 100.0
}

pub fn used_percentage(available: u64, total: u64) -> f64 {
    let used = total - available;
    (used as f64 / total as f64) * 100.0
}