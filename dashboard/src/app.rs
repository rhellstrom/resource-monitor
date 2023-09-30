use std::collections::HashMap;
use ratatui::widgets::{ListItem, ListState, ScrollbarState};
use crate::server::Server;
use crate::util::{used_as_percentage};

pub struct App {
    pub title: String,
    pub tick_rate: u64,
    pub tabs: TabsState,
    pub should_quit: bool,
    pub servers: Vec<Server>,
    pub scroll: ScrollState,
    pub cpu_chart_data: HashMap<usize, Vec<u64>>,
    pub ram_chart_data: HashMap<usize, Vec<u64>>,
    pub max_chart_data_points: usize,
    pub cpu_list_state: StatefulList,
}

impl App {
    pub fn new(title: String, tick_rate: u64) -> App {
        App {
            title,
            tick_rate,
            tabs: TabsState::new(),
            should_quit: false,
            servers: vec![],
            scroll: ScrollState::new(),
            cpu_chart_data: HashMap::new(),
            ram_chart_data: HashMap::new(),
            max_chart_data_points: (60 * 1000 / tick_rate) as usize,
            cpu_list_state: StatefulList::new(),
        }
    }

    pub fn on_key(&mut self, c: char){
        if c == 'q' {
            self.should_quit = true;
        }
    }

    pub fn on_left(&mut self){
        self.tabs.previous();
    }

    pub fn on_right(&mut self){
        self.tabs.next();
    }

    pub fn on_up(&mut self) {
        self.scroll.up()
    }

    pub fn on_down(&mut self) {
        self.scroll.down()
    }

    pub fn on_tick(&mut self, servers: Vec<Server>) {
        self.tabs.update_tabs(&servers);    //Would prefer static tab names
        self.servers = servers;
        self.update_cpu_chart_data();
        self.update_ram_chart_data();
    }

    //TODO: Make the following functions into something more generic to avoid repetition
    /// Pushes last cpu_data into the vector held in our hashmap and removes all data older than 60 seconds
    pub fn update_cpu_chart_data(&mut self){
        for (i, server) in self.servers.iter().enumerate() {
            if i < self.servers.len() {
                let chart_data = self.cpu_chart_data
                    .entry(i)
                    .or_insert_with(|| vec![0; self.max_chart_data_points]);

                chart_data.push(server.cpu_usage as u64);
                if chart_data.len() > self.max_chart_data_points {
                    let index = chart_data.len() - self.max_chart_data_points;
                    chart_data.drain(..index);
                }
            }
        }
    }
    /// Pushes last RAM data into the vector held in our hashmap and removes all data older than 60 seconds
    pub fn update_ram_chart_data(&mut self){
        for (i, server) in self.servers.iter().enumerate() {
            if i < self.servers.len() {
                let chart_data = self.ram_chart_data
                    .entry(i)
                    .or_insert_with(|| vec![0; self.max_chart_data_points]);

                chart_data.push(used_as_percentage(server.used_memory as f64, server.total_memory as f64) as u64);
                if chart_data.len() > self.max_chart_data_points {
                    let index = chart_data.len() - self.max_chart_data_points;
                    chart_data.drain(..index);
                }
            }
        }
    }
}

pub struct ScrollState {
    pub vertical_scroll_state: ScrollbarState,
    pub scroll_pos: u16,
    pub scroll_content_length: u16,
}

impl ScrollState{
    pub fn new() -> ScrollState {
        ScrollState{
            vertical_scroll_state: Default::default(),
            scroll_pos: 0,
            scroll_content_length: 0,
        }
    }

    pub fn up(&mut self){
        if self.scroll_pos > 0 {
            self.scroll_pos -= 1;
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.scroll_pos);
        }
    }

    pub fn down(&mut self) {
        if self.scroll_pos < self.scroll_content_length {
            self.scroll_pos += 1;
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.scroll_pos);
        }
    }
}

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new() -> TabsState {
        TabsState { titles: vec![], index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
    pub fn update_tabs(&mut self, servers: &Vec<Server>) {
        let mut names: Vec<String> = vec![];
        names.push(String::from("Overview"));
        for server in servers{
            names.push(server.hostname.chars().take(10).collect());
        }
        self.titles = names;
    }
}

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
}

impl StatefulList{
    pub fn new() -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items: vec![],
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}





