use std::fs::File;
use std::io;
use std::io::BufRead;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

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
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
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
    time_units.join(", ")
}

pub fn log_scale(value: f64, max_value: f64) -> f64 {
    if value <= 0.0 {
        0.0
    } else {
        ((value + 1.0).log10() / (max_value + 1.0).log10()) * 100.0
    }
}

pub fn format_kilobytes(kilobytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * KB;
    const GB: u64 = KB * MB;

    if kilobytes < KB {
        format!("{} KB", kilobytes)
    } else if kilobytes < MB {
        format!("{:.2} MB", kilobytes as f64 / KB as f64)
    } else if kilobytes < GB {
        format!("{:.2} GB", kilobytes as f64 / MB as f64)
    } else {
        format!("{:.2} TB", kilobytes as f64 / GB as f64)
    }
}

/// Takes file path(s) as parameter, iterates over each file and return a vector of endpoints.
/// Each endpoint should be separated by newline
pub fn extract_endpoints_from_files(file_paths: Vec<String>) -> Vec<String> {
    let mut endpoints: Vec<String> = Vec::new();
    for file_path in file_paths {
        match File::open(&file_path) {
            Err(_) => {
                eprintln!("Failed to open file: {}", file_path);
                continue;
            }
            Ok(file) => {
                let reader = io::BufReader::new(file);
                for line in reader.lines().flatten() {
                    endpoints.push(line);
                }
            }
        };
    }

    endpoints
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

//Converts Kilobytes per second to Megabits per second
pub fn kbs_to_mbps(kb_per_sec: u64) -> f64 {
    // 1 KB = 8 Kb
    let kb_to_kb = kb_per_sec as f64 * 8.0;
    // 1 Mb = 1000 Kb
    kb_to_kb / 1000.0
}
