mod app;
mod scramble;
mod stats;
mod timer;
mod ui;

use crate::app::App;
use anyhow::Result;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use scramble::Scramble;
use std::io;
use std::panic;
use std::time::Duration;
use std::time::Instant;
use ui::render;

fn main() -> Result<()> {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        reset_terminal().unwrap();
        default_hook(info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();
    let res = run_tui(&mut terminal, &mut app);

    reset_terminal()?;

    if let Err(e) = res {
        eprintln!("{e:?}");
    }

    Ok(())
}

fn run_tui<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        if app.holding_space_count == 2 {
            app.timer.reset();
        }

        terminal.draw(|f| render(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => app.scramble = Scramble::new(25),
                    KeyCode::Char(' ') => {
                        if app.timer.is_running() {
                            app.stop_timer();
                        } else {
                            app.holding_space_count += 1;
                        }
                    }
                    _ => (),
                }
            }
            last_tick = Instant::now();
        } else if app.holding_space_count > 1 {
            app.timer.start();
            app.holding_space_count = 0;
        } else {
            app.holding_space_count = 0;
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
