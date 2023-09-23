/// Shows the total usage as percentage
pub fn used_as_percentage(used: u64, total: u64) -> f64 {
    (used as f64 / total as f64) * 100.0
}

/// Calculate how much space is used and returns it as percentage
pub fn used_percentage(available: u64, total: u64) -> f64 {
    let used = total - available;
    (used as f64 / total as f64) * 100.0
}

/// Bytes to GiB
pub fn bytes_to_gib(bytes: u64) -> f64 {
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0; // 1 GiB
    bytes as f64 / GIB
}
