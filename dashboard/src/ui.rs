use std::io::Stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal};
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;
use crate::server::Server;
use crate::util::{total_memory, used_percentage};

pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App){
    let chunks = Layout::default()
        .constraints([Constraint::Length(4), Constraint::Min(0)].as_ref())
        .split(f.size());
    draw_tabs(f, app, chunks[0]);
    draw_server_overview(f, app, chunks[1]);
}

pub fn draw_tabs(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App, area: Rect){
    let titles: Vec<Line> = app
        .tabs
        .titles
        .iter()
        .map(| t| {
            text::Line::from(Span::styled(
                t,
                Style::default().fg(Color::Green)))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL)
            .title(app.title.clone()))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, area);
}

pub fn draw_gauge(f: &mut Frame<CrosstermBackend<Stdout>>, percentage: u16, title: &str, area: Rect) {
    let gauge_style = if percentage > 90 {
        Style::default().fg(Color::Red)
    } else if percentage > 80 {
        Style::default().fg(Color::LightYellow)
    } else {
        Style::default().fg(Color::Green)
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .gauge_style(gauge_style)
        .percent(percentage);
    f.render_widget(gauge, area);
}


pub fn draw_server(f: &mut Frame<CrosstermBackend<Stdout>>, server: &Server, area: Rect) {
    let chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref()) // Adjust the length of the first column as needed
        .margin(0)
        .split(area);
    let block = Block::default().borders(Borders::ALL).title(server.hostname.clone());
    f.render_widget(block, area);

    let gauge_constraints = vec![Constraint::Percentage(20); 5]; // Adjust the percentage as needed
    let gauge_chunks = Layout::default()
        .direction(Horizontal)
        .constraints(gauge_constraints)
        .margin(2)
        .split(chunks[1]);

    draw_gauge(f, server.cpu_usage as u16, "CPU Usage", gauge_chunks[0]);
    draw_gauge(f, total_memory(server.used_memory, server.total_memory) as u16,
               "Memory Usage", gauge_chunks[1]);
    draw_gauge(f, used_percentage(server.available_space, server.total_space) as u16,
               "Disk Usage", gauge_chunks[2]);
    draw_gauge(f, 85, "w/e Usage", gauge_chunks[3]);
    draw_gauge(f, 96, "w/e Usage", gauge_chunks[4]);
}

pub fn draw_server_overview(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App, area: Rect) {
    // Calculate the height of each subarea
    let no_of_servers = app.servers.len() as u16;
    if no_of_servers > 0 {
        let subarea_height = area.height / no_of_servers;

        for i in 0..no_of_servers {
            // Calculate the position and dimensions of each subarea
            let subarea = Rect {
                x: area.x,
                y: area.y + i * subarea_height,
                width: area.width,
                height: subarea_height,
            };
            draw_server(f, &app.servers[i as usize], subarea);
        }
    }
}