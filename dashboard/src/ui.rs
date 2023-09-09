use std::io::Stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App){
    let chunks = Layout::default()
        .constraints([Constraint::Length(4), Constraint::Min(0)].as_ref())
        .split(f.size());
    draw_tabs(f, app, chunks[0]);
    //draw_server_overview(f, app, chunks[1]);
    draw_multiple_server_overviews(f, app, chunks[1])
    //draw_gauge(f, app, chunks[1]);
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

pub fn draw_gauge(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(2)])
        .direction(Vertical)
        .margin(1)
        .split(area);

    let test_percentage = match app.servers.get(0) {
        Some(server) => server.cpu_usage as u16,
        None => 0,
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("CPU Usage: "))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(test_percentage);
    f.render_widget(gauge, chunks[0]);
}


pub fn draw_server_overview(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App, area: Rect) {
    let test_percentage = match app.servers.get(0) {
        Some(server) => server.cpu_usage as u16,
        None => 0,
    };

    let chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref()) // Adjust the length of the first column as needed
        .margin(0)
        .split(area);
    let block = Block::default().borders(Borders::ALL).title("Hostname");
    f.render_widget(block, area);

    let gauge_constraints = vec![Constraint::Percentage(20); 5]; // Adjust the percentage as needed
    let gauge_chunks = Layout::default()
        .direction(Horizontal)
        .constraints(gauge_constraints)
        .margin(2)
        .split(chunks[1]);

    for i in 0..5 {
        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("CPU Usage: "))
            .gauge_style(Style::default().fg(Color::Green))
            .percent(test_percentage);

        f.render_widget(gauge, gauge_chunks[i]);
    }
}

pub fn draw_multiple_server_overviews(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App, area: Rect) {
    // Calculate the height of each subarea
    let subarea_height = area.height / 5;

    for i in 0..5 {
        // Calculate the position and dimensions of each subarea
        let subarea = Rect {
            x: area.x,
            y: area.y + i * subarea_height,
            width: area.width,
            height: subarea_height,
        };

        draw_server_overview(f, app, subarea);
    }
}

/*
pub fn draw_overview(f: &mut Frame<CrosstermBackend<Stdout>>, servers: Vec<Server>, area: Rect){

}

pub fn draw_server(f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect){

}

*/