mod app;
mod cube;
mod history;
mod inspection;
mod scramble;
mod stats;
mod timer;
mod ui;

use crate::app::App;
use anyhow::Result;
use app::AppState;
use app::Confirmation;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
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
        if matches!(app.state, AppState::Set) {
            app.timer.reset();
        }

        terminal.draw(|f| render(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                if app.confirmation.is_some() {
                    match key.code {
                        KeyCode::Char('y') => match app.confirmation {
                            Some(Confirmation::Solve) => app.delete_last_solve()?,
                            Some(Confirmation::Session) => app.delete_session()?,
                            _ => (),
                        },
                        KeyCode::Char('n') => app.confirmation = None,
                        _ => (),
                    }
                } else {
                    match key.code {
                        KeyCode::Esc => app.cancel_timer(),
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => app.generate_scramble(),
                        KeyCode::Char('i') => app.toggle_inspection(),
                        KeyCode::Char('d') => app.delete_last_solve()?,
                        KeyCode::Char('p') => app.toggle_plus_two()?,
                        KeyCode::Char('D') => app.toggle_dnf()?,
                        KeyCode::Char('c') => app.delete_session()?,
                        KeyCode::Char('s') => app.next_session(),
                        KeyCode::Char('S') => app.previous_session(),
                        KeyCode::Char(' ') => match app.state {
                            AppState::Idle if !app.inspection.expired => {
                                if app.inspection_enabled && !app.inspection.is_running() {
                                    app.start_inspecting();
                                }

                                app.state = AppState::Ready;
                            }
                            AppState::Ready => app.state = AppState::Set,
                            AppState::Solving => {
                                app.stop_timer()?;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            last_tick = Instant::now();
        } else {
            app.inspection.expired = false;
            match app.state {
                AppState::Set => {
                    app.inspection.stop();
                    app.start_timer();
                }
                AppState::Ready => app.state = AppState::Idle,
                _ => (),
            }
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
