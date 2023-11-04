use std::collections::HashMap;
use std::time::{Duration, Instant};
use ratatui::widgets::{ScrollbarState, TableState};
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
    pub cpu_table: CpuTable,
    pub previous_transmitted_total: HashMap<usize, u64>,
    pub previous_received_total: HashMap<usize, u64>,
    pub received_chart_data: HashMap<usize, Vec<f64>>,
    pub transmitted_chart_data: HashMap<usize, Vec<f64>>,
    pub last_update_time: Instant,
    pub update_interval: u64,
    pub show_endpoint_popup: bool,
    pub endpoint_input: InputState,
}

impl App {
    pub fn new(title: String, tick_rate: u64, update_interval: u64) -> App {
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
            cpu_table: CpuTable::new(),
            previous_transmitted_total: HashMap::new(),
            previous_received_total: HashMap::new(),
            received_chart_data: HashMap::new(),
            transmitted_chart_data:HashMap::new(),
            last_update_time: Instant::now(),
            update_interval,
            show_endpoint_popup: false,
            endpoint_input: InputState::default(),
        }
    }

    pub fn on_key(&mut self, c: char){
        if c == 'q' {
            self.should_quit = true;
        }
        if c == 'p' {
            self.show_endpoint_popup = !self.show_endpoint_popup;
        }
    }

    pub fn on_left(&mut self){
        self.tabs.previous();
        self.cpu_table.state.select(Some(0));
    }

    pub fn on_right(&mut self){
        self.tabs.next();
        self.cpu_table.state.select(Some(0));
    }

    pub fn on_up(&mut self) {
        if self.tabs.index == 0 {
            self.scroll.up()
        }
        else {
            self.cpu_table.previous();
        }
    }

    pub fn on_down(&mut self) {
        if self.tabs.index == 0 {
            self.scroll.down()
        }
        else {
            self.cpu_table.next();
        }
    }

    pub fn on_tick(&mut self, servers: Vec<Server>) {
        self.tabs.update_tabs(&servers);
        self.update_cpu_chart_data();
        self.update_ram_chart_data();
        self.update_network_chart_data();

        if self.last_update_time.elapsed() >= Duration::from_millis(self.update_interval){
            self.update_previous_network_data();
            self.servers = servers;
            self.last_update_time = Instant::now();
        }
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

    //Saves the previous value for chart comparison
    pub fn update_previous_network_data(&mut self) {
        for (i, server) in self.servers.iter().enumerate() {
            let old_transmitted = server.bytes_transmitted;
            let old_received = server.bytes_received;

            self.previous_transmitted_total.insert(i, old_transmitted);
            self.previous_received_total.insert(i, old_received);
        }
    }

    pub fn update_network_chart_data(&mut self) {
        for (i, server) in self.servers.iter().enumerate() {
            if i < self.servers.len() {
                let transmitted_data = self.transmitted_chart_data
                    .entry(i)
                    .or_insert_with(|| vec![0.0; self.max_chart_data_points]);

                let received_data = self.received_chart_data
                    .entry(i)
                    .or_insert_with(|| vec![0.0; self.max_chart_data_points]);

                let previous_received = self.previous_received_total.get(&i);
                let previous_transmitted = self.previous_transmitted_total.get(&i);

                if let (Some(previous_received_total), Some(previous_transmitted_total)) = (previous_received, previous_transmitted) {
                    let tick_rate_sec = self.tick_rate as f64 / 1000.0;
                    let rx_kb_per_sec = ((server.bytes_received - previous_received_total) as f64) / tick_rate_sec / 1024.0;
                    let tx_kb_per_sec = ((server.bytes_transmitted - previous_transmitted_total) as f64) / tick_rate_sec / 1024.0;

                    //To avoid showing the total rx/tx on our first calculation
                    if *previous_received_total != 0 && *previous_transmitted_total != 0 {
                        received_data.push(rx_kb_per_sec);
                        transmitted_data.push(tx_kb_per_sec);
                    }

                }
                if transmitted_data.len() > self.max_chart_data_points {
                    transmitted_data.drain(..transmitted_data.len() - self.max_chart_data_points);
                }
                if received_data.len() > self.max_chart_data_points {
                    received_data.drain(..received_data.len() - self.max_chart_data_points);
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
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.scroll_pos as usize);
        }
    }

    pub fn down(&mut self) {
        if self.scroll_pos < self.scroll_content_length {
            self.scroll_pos += 1;
            self.vertical_scroll_state = self.vertical_scroll_state.position(self.scroll_pos as usize);
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


pub struct CpuTable {
    pub state: TableState,
    pub size: usize,
}

impl CpuTable {
    pub fn new() -> CpuTable {
        CpuTable {
            state: TableState::default(),
            size: 0,
        }
    }

    pub fn next(&mut self) {
        if self.size == 0 { return;}

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.size - 1 {
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
        if self.size == 0 { return;}
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.size - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct InputState{
    pub input: String,
    pub cursor_position: usize,
}

impl InputState {
    fn default() -> InputState {
        InputState{
            input: String::new(),
            cursor_position: 0,
        }
    }

    pub(crate) fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub(crate) fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub(crate) fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    pub(crate) fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    //TODO: INIT A SERVER FROM ENDPOINT INPUT AND PUSH TO Vec<Server>
    pub(crate) fn add_endpoint(&mut self) -> Server {
        let new_server = Server::new(self.input.clone());
        self.input.clear();
        self.reset_cursor();
        new_server
    }
}