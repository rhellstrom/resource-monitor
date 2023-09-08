use std::io::Stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::App;

pub fn draw(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App){
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    draw_tabs(f, app, chunks[0]);
    draw_gauge(f, app, chunks[1]);
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
        .direction(Direction::Vertical)
        .margin(1)
        .split(area);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("CPU Usage: "))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.test_cpu as u16);
    f.render_widget(gauge, chunks[0]);
}

/*
pub fn draw_overview(f: &mut Frame<CrosstermBackend<Stdout>>, servers: Vec<Server>, area: Rect){

}

pub fn draw_server(f: &mut Frame<CrosstermBackend<Stdout>>, area: Rect){

}

let current_cpu = servers.get(0).unwrap().cpu_usage;

*/