use std::collections::HashMap;
use ratatui::widgets::ScrollbarState;
use crate::server::Server;

pub struct App {
    pub title: String,
    pub tabs: TabsState,
    pub should_quit: bool,
    pub servers: Vec<Server>,
    pub vertical_scroll_state: ScrollbarState,
    pub scroll_pos: u16,
    pub scroll_content_length: u16,
    pub cpu_sparkline_data: HashMap<usize, Vec<u64>>,
}

impl App {
    pub fn new(title: String) -> App {
        App {
            title,
            tabs: TabsState::new(),
            should_quit: false,
            servers: vec![],
            vertical_scroll_state: Default::default(),
            scroll_pos: 0,
            scroll_content_length: 0,
            cpu_sparkline_data: HashMap::new(),
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
        //vertical_scroll_state.prev() would be great but fields are private
        if self.scroll_pos > 0 {
            self.scroll_pos -= 1;
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.scroll_pos);
        }
    }

    pub fn on_down(&mut self) {
        //vertical_scroll_state.next() would be great but fields are private
        if self.scroll_pos < self.scroll_content_length {
            self.scroll_pos += 1;
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.scroll_pos);
        }
    }

    pub fn on_tick(&mut self, servers: Vec<Server>) {
        self.tabs.update_tabs(&servers);    //Would prefer static tab names
        self.servers = servers;
        self.update_cpu_sparkline();
    }

    pub fn update_cpu_sparkline(&mut self){
        for (i, server) in self.servers.iter().enumerate() {
            // Ensure the server index is within bounds
            if i < self.servers.len() {
                let sparkline_data = self.cpu_sparkline_data
                    .entry(i)
                    .or_insert_with(Vec::new);

                // Add the server's CPU usage and keep at most 20 values
                sparkline_data.push(server.cpu_usage as u64);
                if sparkline_data.len() > 200 {
                    sparkline_data.remove(0);
                }
            }
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




