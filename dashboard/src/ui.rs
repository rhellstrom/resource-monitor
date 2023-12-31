use std::ops::Index;
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal};
use ratatui::prelude::*;
use ratatui::style::Color::{Magenta, Yellow};
use ratatui::widgets::*;
use ratatui::widgets::block::{Position, Title};
use crate::app::App;
use crate::util::{bytes_to_gb, bytes_to_gib, centered_rect, format_kilobytes, format_seconds, kbs_to_mbps, log_scale, used_as_percentage, used_percentage};

pub fn draw(f: &mut Frame, app: &mut App){
    let chunks = Layout::default()
        .constraints([Constraint::Length(4), Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());
    draw_tabs(f, app, chunks[0]);

    if app.tabs.index == 0 {
        draw_server_overview(f, app, chunks[1]);
    }
    else {
        draw_detailed_view(f, app, chunks[1]);
    }

    if app.show_endpoint_popup {
        draw_endpoint_popup(f, app);
    }

    draw_key_legend(f, app, chunks[2]);
}

fn draw_tabs(f: &mut Frame, app: &mut App, area: Rect){
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
        .highlight_style(Style::default().fg(Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, area);
}

fn draw_gauge(f: &mut Frame, percentage: u16, title: &str, area: Rect) {
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

fn draw_server(f: &mut Frame, app: &mut App, index: usize, area: Rect) {
    let server = app.servers.get(index).unwrap();
    let block = Block::default().borders(Borders::ALL).title(server.hostname.clone());
    f.render_widget(block, area);

    let gauge_constraints = vec![Constraint::Percentage(20); 5];
    let gauge_chunks = Layout::default()
        .direction(Horizontal)
        .constraints(gauge_constraints)
        .margin(2)
        .split(area);

    let uptime = Paragraph::new(format_seconds(server.uptime))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Uptime")
            .padding(Padding::new(0, 0, 1, 0 )))
        .alignment(Alignment::Center);

    if let Some(received_data) = app.received_chart_data.get(&(index)) {
        if let Some(transmitted_data) = app.transmitted_chart_data.get(&(index)) {
            let rx = received_data.last().unwrap();
            let tx = transmitted_data.last().unwrap();
            let network = Paragraph::new(format!("{:.1} Mbps | {:.1} Mbps", kbs_to_mbps(*rx as u64), kbs_to_mbps(*tx as u64)))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Network I/O")
                    .padding(Padding::new(0, 0, 1, 0 )))
                .alignment(Alignment::Center);
            f.render_widget(network, gauge_chunks[4]);
        }
    }

    f.render_widget(uptime, gauge_chunks[0]);

    draw_gauge(f, server.cpu_usage as u16, "CPU Usage", gauge_chunks[1]);
    draw_gauge(f, used_as_percentage(server.used_memory as f64, server.total_memory as f64) as u16,
               "Memory Usage", gauge_chunks[2]);
    draw_gauge(f, used_percentage(server.available_space, server.total_space) as u16,
               "Disk Usage", gauge_chunks[3]);

}

fn draw_server_overview(f: &mut Frame, app: &mut App, area: Rect) {
    let no_of_servers = app.servers.len() as u16;
    app.scroll.scroll_content_length = no_of_servers;
    if no_of_servers > 0 {
        let position = app.scroll.scroll_pos;
        let terminal_height = f.size().height;
        let view_length_fraction = 0.12;  // Adjust this to find a suitable size
        let view_length = (terminal_height as f64 * view_length_fraction) as u16;
        let end_index = position + view_length.min(no_of_servers - position);
        let subarea_height = area.height / view_length;
        app.scroll.vertical_scroll_state = app.scroll.vertical_scroll_state.content_length(app.scroll.scroll_content_length as usize);
        app.scroll.vertical_scroll_state = app.scroll.vertical_scroll_state.viewport_content_length(view_length as usize);

        for i in position ..end_index {
            // Calculate the position and dimensions of each subarea
            let subarea = Rect {
                x: area.x,
                y: area.y + (i - position) * subarea_height,
                width: area.width,
                height: subarea_height,
            };
            if i < no_of_servers {
                draw_server(f,  app, i as usize, subarea);
            }
        }
        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut app.scroll.vertical_scroll_state,
        );
    }
}

fn draw_detailed_view(f: &mut Frame, app: &mut App, area: Rect) {
    let current_index = app.tabs.index - 1; // The overview is always first index
    let block = Block::default().borders(Borders::ALL).title(app.servers.index(current_index).hostname.clone());
    f.render_widget(block, area);

    let chunk_height = area.height / 3;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(chunk_height); 3].as_ref())
        .split(area);

    draw_cpu_row(f, app, chunks[0]);
    draw_memory_row(f, app, chunks[1]);
    draw_info_network_row(f, app, chunks[2]);
}

fn draw_cpu_row(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
        .margin(0)
        .split(area);

    draw_cpu_chart(f, app, chunks[0]);
    draw_cpu_table(f, app, chunks[1]);
}

fn draw_memory_row(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .margin(0)
        .split(area);

    draw_ram_chart(f, app, chunks[0]);
    draw_disk_table(f, app, chunks[1]);
}

fn draw_info_network_row(f: &mut Frame, app: &mut App, area: Rect){
    let chunks = Layout::default()
        .direction(Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .margin(0)
        .split(area);

    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    draw_network_chart(f, app, chunks[0]);
    draw_network_info_list(f, app, sub_chunks[0]);
    draw_info_list(f, app, sub_chunks[1]);
}

fn draw_ram_chart(f: &mut Frame, app: &mut App, area: Rect) {
    let current_tab_index = app.tabs.index;
    if let Some(ram_data) = app.ram_chart_data.get(&(current_tab_index - 1)) {
        let used = bytes_to_gib(app.servers.get(current_tab_index - 1).unwrap().used_memory);
        let total = bytes_to_gib(app.servers.get(current_tab_index - 1).unwrap().total_memory);
        let percentage = used_as_percentage(used, total);

        let data: Vec<(f64, f64)> = ram_data
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64, val as f64))
            .collect();

        let dataset = vec![
            Dataset::default()
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(Color::Blue))
                .data(&data)];

        let chart = Chart::new(dataset)
            .block(Block::new()
                .borders(Borders::ALL)
                .title(
                    Title::from("RAM")
                        .position(Position::Top)
                        .alignment(Alignment::Left),
                )
                .title(
                    Title::from(format!("{:.1}%\t {:.1}GiB/{:.1}GiB", percentage, used, total))
                        .position(Position::Top)
                        .alignment(Alignment::Right),
                ))
            .x_axis(Axis::default()
                .bounds([0.0, ram_data.len() as f64 - 1.0])
                .labels(["60s", "0s"].iter().cloned().map(Span::from).collect()))
            .y_axis(Axis::default()
                .bounds([0.0, 100.0])
                .labels(["0%", "100%"].iter().cloned().map(Span::from).collect()));

        f.render_widget(chart, area);
    }
}

fn draw_cpu_chart(f: &mut Frame, app: &mut App, area: Rect){
    let current_server_index = app.tabs.index - 1;
    if let Some(cpu_data) = app.cpu_chart_data.get(&(current_server_index)) {
        let one = app.servers.get(current_server_index).unwrap().load_avg_one;
        let five = app.servers.get(current_server_index).unwrap().load_avg_five;
        let fifteen = app.servers.get(current_server_index).unwrap().load_avg_fifteen;
        let usage = app.servers.get(current_server_index).unwrap().cpu_usage;

        let data: Vec<(f64, f64)> = cpu_data
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64, val as f64))
            .collect();

        let dataset = vec![Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Green))
            .data(&data)];

        let chart = Chart::new(dataset)
            .block(Block::default()
                .title(Title::from("CPU")
                            .position(Position::Top)
                            .alignment(Alignment::Left))
                .title(Title::from(format!("{:.2} {:.2} {:.2}", one, five, fifteen))
                            .position(Position::Top)
                            .alignment(Alignment::Right))
                .title(Title::from(format!("{:.1}%", usage))
                            .position(Position::Top)
                            .alignment(Alignment::Right)
                )
                .borders(Borders::ALL))
            .x_axis(Axis::default()
                .bounds([0.0, (data.len() - 1) as f64])
                .labels(["60s", "0s"].iter().cloned().map(Span::from).collect()))
            .y_axis(Axis::default()
                .bounds([0.0, 100.0])
                .labels(["0%", "100%"].iter().cloned().map(Span::from).collect()));

        f.render_widget(chart, area);
    }
}


fn draw_cpu_table(f: &mut Frame, app: &mut App, area: Rect){
    let server_index = app.tabs.index - 1;
    let load_per_cores = &app.servers.get(server_index).unwrap().cpu_load_per_core;

    let mut rows: Vec<Row> = vec![];
    let header_row = Row::new(vec!["CPU", "Use"])
        .style(Style::default())
        .height(1);

    for (i, &load) in load_per_cores.iter().enumerate() {
        let mut cpu_core_row = vec![];
        cpu_core_row.push(i.to_string());
        cpu_core_row.push(format!("{:.1}%", load));
        rows.push(Row::new(cpu_core_row.clone()));
    }
    app.cpu_table.size = rows.len();

    let table = Table::new(rows)
        .header(header_row)
        .block(Block::default()
            .borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]);
    f.render_stateful_widget(table, area, &mut app.cpu_table.state);
}

fn draw_disk_table(f: &mut Frame, app: &mut App, area: Rect){
    let server_index = app.tabs.index - 1;
    let header_row = Row::new(vec!["Disk", "Used", "Free", "Total"])
        .style(Style::default())
        .bottom_margin(1)
        .height(1);
    let mut rows: Vec<Row> = vec![];

    let disk_names = app.servers.get(server_index).unwrap().disk_names.clone();
    let disk_available = &app.servers.get(server_index).unwrap().disk_available;
    let disk_total = &app.servers.get(server_index).unwrap().disk_total;

    for (i, disk) in disk_names.iter().enumerate() {
        let disk_used = disk_total[i] - disk_available[i];
        let mut disk_row = vec![];
        disk_row.push(disk.to_string());
        disk_row.push(format!("{:.1}GB", bytes_to_gb(disk_used)));
        disk_row.push(format!("{:.1}GB", bytes_to_gb(disk_available[i])));
        disk_row.push(format!("{:.1}GB", bytes_to_gb(disk_total[i])));
        rows.push(Row::new(disk_row));
    }

    let table = Table::new(rows)
        .header(header_row)
        .block(Block::default()
            .borders(Borders::ALL))
        .widths(&[
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20)
        ]);

    f.render_widget(table, area);
}

fn draw_info_list(f: &mut Frame, app: &mut App, area: Rect){
    let server_index = app.tabs.index - 1;
    let mut items: Vec<ListItem> = vec![];
    items.push(ListItem::new(format!("OS: {}", app.servers.get(server_index).unwrap().os_version)));
    items.push(ListItem::new(format!("Kernel: {}", app.servers.get(server_index).unwrap().kernel_version)));
    items.push(ListItem::new(format!("Hostname: {}", app.servers.get(server_index).unwrap().hostname)));
    items.push(ListItem::new(format!("Uptime: {}", format_seconds(app.servers.get(server_index).unwrap().uptime))));

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL));
    f.render_widget(list, area);
}


fn draw_network_chart(f: &mut Frame, app: &App, area: Rect) {
    let current_server_index = app.tabs.index - 1;
    if let Some(received_data) = app.received_chart_data.get(&(current_server_index)) {
        if let Some(transmitted_data) = app.transmitted_chart_data.get(&(current_server_index)){
            let greeting = Paragraph::new(format!("RX: {} KB/S   TX: {} KB/S RX TOTAL: {} TX TOTAL: {}",
                                                  received_data.last().unwrap(),
                                                  transmitted_data.last().unwrap(),
                                                  app.servers.get(current_server_index).unwrap().bytes_received,
                                                  app.servers.get(current_server_index).unwrap().bytes_transmitted));
            f.render_widget(greeting, area);
            let mut max_rx = *received_data.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0);
            if max_rx < 100.0 { max_rx = 100.0;}
            let scaled_rx: Vec<f64> = received_data.iter().map(|&val| log_scale(val, max_rx)).collect();
            let scaled_tx: Vec<f64> = transmitted_data.iter().map(|&val| log_scale(val, max_rx)).collect();

            let max_string= format!("{}/s", format_kilobytes(max_rx as u64));
            let my_str_ref: &str = max_string.as_str();

            let rx : Vec<(f64, f64)> = scaled_rx
                .iter()
                .enumerate()
                .map(|(i, &val)| (i as f64, val))
                .collect();
            let tx : Vec<(f64, f64)> = scaled_tx
                .iter()
                .enumerate()
                .map(|(i, &val)| (i as f64, val))
                .collect();

            let datasets = vec![
                Dataset::default()
                    .marker(Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Magenta))
                    .data(&rx),
                Dataset::default()
                    .marker(Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Yellow))
                    .data(&tx),
            ];

            let chart = Chart::new(datasets)
                .block(Block::default()
                    .title(Title::from("Network")
                        .position(Position::Top)
                        .alignment(Alignment::Left))
                    .borders(Borders::ALL))

                .x_axis(Axis::default()
                    .bounds([0.0, (received_data.len() - 1) as f64])
                    .labels(["60s", "0s"].iter().cloned().map(Span::from).collect()))
                .y_axis(Axis::default()
                    .bounds([0.0, 100.0])
                    .labels(["0 KB/s", my_str_ref].iter().cloned().map(Span::from).collect()));

            f.render_widget(chart, area);
        }
    }
}

fn draw_network_info_list(f: &mut Frame, app: &mut App, area: Rect){
    let current_server_index = app.tabs.index - 1;
    if let Some(received_data) = app.received_chart_data.get(&(current_server_index)) {
        if let Some(transmitted_data) = app.transmitted_chart_data.get(&(current_server_index)) {
            let mut items: Vec<ListItem> = vec![];
            items.push(ListItem::new(format!("RX: {}/s", format_kilobytes(*received_data.last().unwrap() as u64)))
                .style(Style::default().fg(Magenta)));
            items.push(ListItem::new(format!("TX: {}/s", format_kilobytes(*transmitted_data.last().unwrap() as u64)))
                .style(Style::default().fg(Yellow)));
            items.push(ListItem::new(format!("Total: {}", format_kilobytes(app.servers.get(current_server_index).unwrap().bytes_received / 1024)))
                .style(Style::default().fg(Magenta)));
            items.push(ListItem::new(format!("Total: {}", format_kilobytes(app.servers.get(current_server_index).unwrap().bytes_transmitted / 1024)))
                .style(Style::default().fg(Yellow)));

            let list = List::new(items)
                .block(Block::default()
                    .title("Network I/O")
                    .borders(Borders::ALL));
            f.render_widget(list, area);
        }
    }
}

fn draw_endpoint_popup(f: &mut Frame, app: &mut App){
    let size = f.size();
    let area = centered_rect(60, 20, size);
    f.render_widget(Clear, area); //this clears out the background

    let input = Paragraph::new(app.endpoint_input.input.as_str())
        .block(Block::default()
            .borders(Borders::ALL)
            .title_alignment(Alignment::Center)
            .title("Enter a server endpoint"));
    f.render_widget(input, area);

    //Set cursor
    f.set_cursor(area.x + app.endpoint_input.cursor_position as u16 + 1,
                 // Move one line down, from the border to the input line
                 area.y + 1,)
}

fn draw_key_legend(f: &mut Frame, app: &mut App, area: Rect){
    let title = if app.show_endpoint_popup{
       "Esc: Cancel \t Enter: Add"
    }else{
       "Navigate: ◄ ▲ ▼ ► \t 'p': Add a server \t 'q': Exit the application"
    };
    let title = Block::default()
        .title(title)
        .title_alignment(Alignment::Center);
    f.render_widget(title, area);
}