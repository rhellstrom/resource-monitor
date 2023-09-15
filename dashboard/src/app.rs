use ratatui::widgets::ScrollbarState;
use crate::server::Server;

pub struct App {
    pub title: String,
    pub tabs: TabsState,
    pub should_quit: bool,
    pub servers: Vec<Server>,
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,


}

impl App {
    pub fn new(title: String) -> App {
        App {
            title,
            tabs: TabsState::new(),
            should_quit: false,
            servers: vec![],
            vertical_scroll_state: Default::default(),
            vertical_scroll: 0,
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

    pub fn on_tick(&mut self, servers: Vec<Server>) {
        self.tabs.update_tabs(&servers);    //Would prefer static tab names
        self.servers = servers;
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




