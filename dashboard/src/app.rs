use crate::server::Server;

/// To extract data from Vec<Server> and functions to retrieve and present data

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        TabsState { titles, index: 0 }
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
    pub fn get_titles(servers: &Vec<Server>) -> Vec<String> {
        let mut names: Vec<String> = vec![];
        names.push(String::from("Overview"));
        for server in servers{
            names.push(server.hostname.clone());
        }
        names
    }
}


pub struct App {
    pub title: String,
    pub tabs: TabsState,
}

impl App {
    pub fn new(title: String, servers: Vec<Server>) -> App {
        let titles = TabsState::get_titles(&servers);
        App {
            title,
            tabs: TabsState::new(titles),
        }
    }
}