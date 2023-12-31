use std::{
    io::{self, Stdout},
    time::Duration,
};

use std::sync::{Arc};
use std::time::Instant;

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyEventKind;
use ratatui::{prelude::*};
use tokio::sync::Mutex;
use crate::app::App;
use crate::server::Server;
use crate::ui;

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

/// Ensures we gracefully restore the terminal in case of panic
pub fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

/// Runs the TUI loop. We setup the terminal environment, draw the application and react to user input
/// and updates the data to be drawn on each tick. Once loop is exited we restore the terminal
pub async fn run(servers: Arc<Mutex<Vec<Server>>>, tick_rate: u64, update_interval: u64) -> Result<()> {
    initialize_panic_handler();
    let mut terminal = setup_terminal()?;
    let tick = Duration::from_millis(tick_rate);
    
    let mut app = App::new(String::from("Dashboard"), tick_rate, update_interval);
    let mut last_tick = Instant::now();
    let mut new_endpoint: Option<Server> = None;

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    //Read keyboard input from here
                    if app.show_endpoint_popup{
                        match key.code {
                            KeyCode::Enter => {
                                new_endpoint = Some(app.endpoint_input.add_endpoint());
                                app.show_endpoint_popup = false;
                            },
                            KeyCode::Char(to_insert) => {
                                app.endpoint_input.enter_char(to_insert);
                            }
                            KeyCode::Backspace => {
                                app.endpoint_input.delete_char();
                            }
                            KeyCode::Left => {
                                app.endpoint_input.move_cursor_left();
                            }
                            KeyCode::Right => {
                                app.endpoint_input.move_cursor_right();
                            }
                            KeyCode::Esc => {
                                app.show_endpoint_popup = false;
                            }
                            _ => {}
                        }
                    }
                    else{
                        match key.code {
                            KeyCode::Char(c) => app.on_key(c),
                            KeyCode::Left => app.on_left(),
                            KeyCode::Right => app.on_right(),
                            KeyCode::Up => app.on_up(),
                            KeyCode::Down => app.on_down(),
                            _ => {}
                        }
                    }
                }
            }
        }
        // On each tick we update the data to be drawn in the next iteration
        if last_tick.elapsed() >= tick {
            if let Some(server) = new_endpoint.take() {
                servers.lock().await.push(server);
            }
            app.on_tick(servers.lock().await.to_vec());
            last_tick = Instant::now();
        }
        if app.should_quit {
            break;
        }
    }
    //Restore when we're done
    restore_terminal(&mut terminal)?;
    Ok(())
}

