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

fn draw_tabs(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App, area: Rect){
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

fn draw_gauge(f: &mut Frame<CrosstermBackend<Stdout>>, percentage: u16, title: &str, area: Rect) {
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


fn draw_server(f: &mut Frame<CrosstermBackend<Stdout>>, server: &Server, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(server.hostname.clone());
    f.render_widget(block, area);

    let gauge_constraints = vec![Constraint::Percentage(20); 5];
    let gauge_chunks = Layout::default()
        .direction(Horizontal)
        .constraints(gauge_constraints)
        .margin(2)
        .split(area);

    draw_gauge(f, server.cpu_usage as u16, "CPU Usage", gauge_chunks[0]);
    draw_gauge(f, total_memory(server.used_memory, server.total_memory) as u16,
               "Memory Usage", gauge_chunks[1]);
    draw_gauge(f, used_percentage(server.available_space, server.total_space) as u16,
               "Disk Usage", gauge_chunks[2]);
    draw_gauge(f, 85, "w/e Usage", gauge_chunks[3]);
    draw_gauge(f, 96, "w/e Usage", gauge_chunks[4]);
}

fn draw_server_overview(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App, area: Rect) {
    let no_of_servers = app.servers.len() as u16;
    app.scroll_content_length = no_of_servers;

    if no_of_servers > 0 {
        let position = app.scroll_pos;
        //TODO: Find a way to to calculate view length based on frame size
        let view_length = 5;

        let end_index = position + view_length.min(no_of_servers - position);
        app.vertical_scroll_state = app.vertical_scroll_state.content_length(app.scroll_content_length);
        app.vertical_scroll_state = app.vertical_scroll_state.viewport_content_length(view_length);


        let subarea_height = area.height / view_length;

        for i in position ..end_index {
            // Calculate the position and dimensions of each subarea
            let subarea = Rect {
                x: area.x,
                y: area.y + (i - position) * subarea_height,
                width: area.width,
                height: subarea_height,
            };
            if i < no_of_servers {
                draw_server(f, &app.servers[i as usize], subarea);
            }
        }

        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut app.vertical_scroll_state,
        );
    }
}