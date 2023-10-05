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

/// Formats seconds to into a dd/hh/mm/ss String
pub fn format_seconds(seconds: u64) -> String {
    let days = seconds / (24 * 3600);
    let hours = (seconds / 3600) % 24;
    let minutes = (seconds / 60) % 60;
    let seconds = seconds % 60;

    let mut time_units = Vec::new();

    if days > 0 {
        time_units.push(format!("{:02} days", days));
    }
    if hours > 0 {
        time_units.push(format!("{:02} hours", hours));
    }
    if minutes > 0 {
        time_units.push(format!("{:02} minutes", minutes));
    }
    if seconds > 0 {
        time_units.push(format!("{:02} seconds", seconds));
    }
    time_units.join(", ")
}