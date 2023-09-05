use std::io::Stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::server::Server;

pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, servers: Vec<Server>){
    let current_cpu = servers.get(0).unwrap().cpu_usage;
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Min(0)].as_ref())
        .split(size);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("CPU Usage: "))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(current_cpu as u16);
    f.render_widget(gauge, chunks[0]);
}