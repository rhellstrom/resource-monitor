# Resource Monitor - A Rust-based Lightweight Server Monitoring Tool

Resource Monitor is a lightweight server monitoring tool designed for simplicity. 

<img src="https://github.com/rhellstrom/resource-monitor/blob/main/dashboard/demo.gif" alt=""/> 

GIF created with [VHS](https://github.com/charmbracelet/vhs) </p>

## Motivation
This project served as my introduction to Rust and was initially developed as a school project. Projects such as [top](https://man7.org/linux/man-pages/man1/top.1.html), [ytop](https://github.com/cjbassi/ytop) and [btm](https://github.com/ClementTsang/bottom) heavily influenced the layout of the dashboard.

## Features
Resource Monitor consists of a TUI Dashboard for real-time system resource metrics written with [Ratatui](https://github.com/ratatui-org/ratatui).
* High level overview of essential server metrics
* Access more comprehensive server data through tabs 
* Load endpoints from file(s) at launch
* Add endpoints during runtime

Additionally, a small web server component to be run on the servers in order to access monitoring data remotely is provided.
Resource Monitor was primarily developed for Linux and macOS platforms.

## Installation And Usage
To run the project locally with [Cargo](https://doc.rust-lang.org/cargo/), follow these steps from the root directory. 
### Web server
Firstly, to start the web server. Navigate into the directory and run with cargo. By default port 3000 is used but can be changed through a command line option.
```
cd resouce-monitor && cargo run -- --port 8080
```
You should now be able to access the data through http://localhost:8080/resources

### Dashboard 
Same thing goes for the dashboard. Either load endpoints from newline separated file(s) as argument or add an endpoint during runtime.

```
cd dashboard && cargo run 
```


### CLI Options
```
Usage: dashboard [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Path(s) for loading endpoints from file

Options:
  -t, --tick rate <milliseconds>
          The UI tick rate [default: 250]
  -u, --update-frequency <milliseconds>
          How often to fetch new data from server endpoints [default: 1000]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Final thoughts
As this project was originally created for a school assignment and involves plenty of shortcuts, I look forward to revisiting the code in the future to enhance its concurrent functionality, implement proper error handling and improve the general layout. Overall, I am content with this project, as it served as an excellent introduction to the Rust ecosystem and provided valuable insights into fundamental aspects of the language. 

## License
[MIT](https://github.com/rhellstrom/resource-monitor/blob/main/LICENSE)