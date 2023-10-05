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

/// Runs the TUI loop. We setup the terminal environment, draw the application and react to user input
/// and updates the data to be drawn on each tick. Once loop is exited we restore the terminal
// TODO: Ensure we restore the terminal if we panic
// TODO: Only ever update app.servers when we've actually fetched information instead of on each tick
pub async fn run(servers: Arc<Mutex<Vec<Server>>>, tick_rate: u64, update_interval: u64) -> Result<()> {
    let mut terminal = setup_terminal()?;
    let tick = Duration::from_millis(tick_rate);
    
    let mut app = App::new(String::from("Dashboard"), tick_rate, update_interval);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    //Read keyboard input from here
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
        // On each tick we update the data to be drawn in the next iteration
        if last_tick.elapsed() >= tick {
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

