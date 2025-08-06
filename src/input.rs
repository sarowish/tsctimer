use crate::app::{App, AppState, Confirmation};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(key: KeyEvent, app: &mut App) -> Result<bool> {
    if key.is_release() {
        if let KeyCode::Char(' ') = key.code {
            on_space_release(app);
        }

        return Ok(false);
    }

    match app.state {
        _ if app.confirmation.is_some() => match key.code {
            KeyCode::Char('y') => match app.confirmation {
                Some(Confirmation::Solve) => app.delete_selected_solve()?,
                Some(Confirmation::Session) => app.delete_session()?,
                _ => (),
            },
            KeyCode::Char('n') => app.confirmation = None,
            _ => (),
        },
        AppState::Idle if !app.inspection.is_running() => (),
        AppState::SolveInfo => return handle_solve_info_key(key, app),
        _ => return handle_timer_key(key, app),
    }

    if let KeyModifiers::CONTROL = key.modifiers {
        match key.code {
            KeyCode::Char('e') => app.session.scroll_down(1, false),
            KeyCode::Char('y') => app.session.scroll_up(1, false),
            KeyCode::Char('d') => app.session.scroll_down_half(),
            KeyCode::Char('u') => app.session.scroll_up_half(),
            KeyCode::Char('f') => app.session.scroll_down_full(),
            KeyCode::Char('b') => app.session.scroll_up_full(),
            _ => (),
        }
    } else {
        match key.code {
            KeyCode::Esc => app.state = AppState::Idle,
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('j') | KeyCode::Down => app.session.next(),
            KeyCode::Char('k') | KeyCode::Up => app.session.previous(),
            KeyCode::Char('g') => app.session.select_first(),
            KeyCode::Char('G') => app.session.select_last(),
            KeyCode::Char('r') => app.generate_scramble(),
            KeyCode::Char('R') => {
                if let Some(scramble) = &app.last_scramble {
                    app.scramble = scramble.clone();
                    app.generate_scramble_preview();
                }
            }
            KeyCode::Char('i') => {
                if app.session.selected_idx().is_some() {
                    app.state = AppState::SolveInfo
                }
            }
            KeyCode::Char('e') => app.inspection_enabled = !app.inspection_enabled,
            KeyCode::Char('E') => {
                app.inspection_warning_enabled = !app.inspection_warning_enabled;
            }
            KeyCode::Char('d') => app.delete_selected_solve()?,
            KeyCode::Char('p') => app.toggle_plus_two()?,
            KeyCode::Char('D') => app.toggle_dnf()?,
            KeyCode::Char('c') => app.delete_session()?,
            KeyCode::Char('s') => app.next_session()?,
            KeyCode::Char('S') => app.previous_session()?,
            KeyCode::Char(' ') => {
                if !app.inspection.has_expired() {
                    if app.inspection_enabled && !app.inspection.is_running() {
                        app.start_inspecting();
                    }

                    app.state = AppState::Ready;
                }
            }
            _ => (),
        }
    }

    Ok(false)
}

fn handle_timer_key(key: KeyEvent, app: &mut App) -> Result<bool> {
    match key.code {
        KeyCode::Esc => app.cancel_timer(),
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char(' ') => match app.state {
            AppState::Idle if app.inspection.is_running() => app.state = AppState::Ready,
            AppState::Ready => {
                app.state = AppState::Set;
                app.timer.reset();
            }
            AppState::Solving => {
                app.stop_timer()?;
            }
            _ => (),
        },
        _ => (),
    }

    Ok(false)
}

fn handle_solve_info_key(key: KeyEvent, app: &mut App) -> Result<bool> {
    match key.code {
        KeyCode::Esc => app.state = AppState::Idle,
        KeyCode::Char('j') | KeyCode::Down => app.session.next(),
        KeyCode::Char('k') | KeyCode::Up => app.session.previous(),
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('i') => app.state = AppState::Idle,
        KeyCode::Char('d') => app.delete_selected_solve()?,
        KeyCode::Char('p') => app.toggle_plus_two()?,
        KeyCode::Char('D') => app.toggle_dnf()?,
        _ => (),
    }

    Ok(false)
}

pub fn on_space_release(app: &mut App) {
    match app.state {
        AppState::Set => {
            app.inspection.stop();
            app.start_timer();
        }
        AppState::Ready => app.state = AppState::Idle,
        _ => (),
    }
}
