mod app;
mod cube;
mod history;
mod input;
mod inspection;
mod scramble;
mod stats;
mod timer;
mod ui;

use anyhow::Result;
use app::{App, AppState};
use crossterm::{
    event::{self, Event, KeyboardEnhancementFlags},
    execute, queue,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use input::{handle_key, on_space_release};
use ratatui::{prelude::CrosstermBackend, DefaultTerminal, Terminal};
use std::io::{self, Write};
use std::panic;
use std::time::{Duration, Instant};
use ui::render;

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    terminal.clear()?;

    let mut app = App::new()?;
    let res = run_tui(&mut terminal, &mut app);

    reset_terminal()?;

    if let Err(e) = res {
        eprintln!("{e:?}");
    }

    Ok(())
}

fn run_tui(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        if !app.inspection.tick(app.inspection_warning_enabled) {
            app.add_solve()?;
            app.state = AppState::Idle;
            app.generate_scramble();
        }

        terminal.draw(|f| render(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = crossterm::event::read()? {
                if handle_key(key, app)? {
                    break;
                }
            }

            last_tick = Instant::now();
            continue;
        } else if !app.supports_keyboard_enhancement {
            on_space_release(app);
        }

        if app.inspection.has_expired() {
            app.inspection.reset();
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    Ok(())
}

fn set_panic_hook() {
    let hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        reset_terminal().unwrap();
        hook(info);
    }));
}

fn init_terminal() -> io::Result<DefaultTerminal> {
    set_panic_hook();
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    queue!(stdout, EnterAlternateScreen)?;

    if terminal::supports_keyboard_enhancement()? {
        queue!(
            stdout,
            event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        )?;
    }

    // `REPORT_EVENT_TYPES` doesn't get enabled if it is placed before `EnterAlternateScreen`
    stdout.flush()?;

    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;

    let mut stdout = io::stdout();

    if terminal::supports_keyboard_enhancement()? {
        queue!(stdout, event::PopKeyboardEnhancementFlags)?;
    }

    execute!(stdout, LeaveAlternateScreen)?;

    Ok(())
}
