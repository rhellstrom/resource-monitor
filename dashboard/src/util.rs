/// Shows the total usage as percentage
pub fn used_as_percentage(used: f64, total: f64) -> f64 {
    (used / total) * 100.0
}

/// Calculate how much space is used and returns it as percentage
pub fn used_percentage(available: u64, total: u64) -> f64 {
    let used = total - available;
    (used as f64 / total as f64) * 100.0
}

pub fn bytes_to_gib(bytes: u64) -> f64 {
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0; // 1 GiB
    bytes as f64 / GIB
}

pub fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_000_000_000.0
}
