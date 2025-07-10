use crate::{
    cube::Cube,
    history,
    inspection::Inspection,
    scramble::Scramble,
    stats::{get_avg, StatEntry, Stats},
    timer::Timer,
};
use anyhow::Result;
use crossterm::terminal;
use ratatui::widgets::TableState;
use std::{
    cmp::Ordering,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const SCRAMBLE_LENGTH: u8 = 25;

pub enum AppState {
    Idle,
    Solving,
    Set,
    Ready,
}

pub enum Confirmation {
    Solve,
    Session,
}

pub struct App {
    pub timer: Timer,
    pub inspection: Inspection,
    pub scramble: Scramble,
    pub last_scramble: Option<Scramble>,
    pub session: Session,
    pub available_sessions: Vec<bool>,
    pub selected_session_idx: usize,
    pub solves_state: TableState,
    pub cube_preview: Cube,
    pub state: AppState,
    pub inspection_enabled: bool,
    pub inspection_warning_enabled: bool,
    pub confirmation: Option<Confirmation>,
    pub supports_keyboard_enhancement: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut app = Self {
            timer: Timer::new(),
            inspection: Inspection::new(),
            scramble: Scramble::new(SCRAMBLE_LENGTH),
            last_scramble: None,
            session: Session::default(),
            available_sessions: Vec::default(),
            selected_session_idx: 0,
            solves_state: TableState::default().with_selected(Some(0)),
            cube_preview: Cube::new(),
            state: AppState::Idle,
            inspection_enabled: true,
            inspection_warning_enabled: true,
            confirmation: None,
            supports_keyboard_enhancement: terminal::supports_keyboard_enhancement()?,
        };

        app.generate_scramble_preview();

        for session_file in history::get_sessions_list().unwrap() {
            let idx = session_file
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .split_once('_')
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap()
                - 1;

            match idx.cmp(&app.available_sessions.len()) {
                Ordering::Less => app.available_sessions[idx] = true,
                Ordering::Equal => app.available_sessions.push(true),
                Ordering::Greater => {
                    for _ in 0..(idx - app.available_sessions.len()) {
                        app.available_sessions.push(false);
                    }
                    app.available_sessions.push(true);
                }
            }
        }

        if app.available_sessions.is_empty() {
            app.available_sessions.push(false);
        } else {
            app.load_session()?;
        }

        Ok(app)
    }

    pub fn load_session(&mut self) -> Result<()> {
        let path = history::get_session_history_file(&format!(
            "session_{}.csv",
            self.selected_session_idx + 1
        ))?;

        let mut session = history::read_history(path).unwrap();

        let mut start = 0;
        let mut end = 4;

        while session.solves.len() > end {
            let slice = &session.solves[start..=end];
            let avg_of_5 = get_avg(slice, 5);
            session.solves[end].avg_of_5 = avg_of_5;
            start += 1;
            end += 1;
        }

        start = 0;
        end = 11;

        while session.solves.len() > end {
            let slice = &session.solves[start..=end];
            let avg_of_12 = get_avg(slice, 12);
            session.solves[end].avg_of_12 = avg_of_12;
            start += 1;
            end += 1;
        }

        session.update_stats();
        self.session = session;

        self.scroll(0);

        Ok(())
    }

    pub fn delete_session(&mut self) -> Result<()> {
        if matches!(self.confirmation, Some(Confirmation::Session)) {
            let path = history::get_session_history_file(&format!(
                "session_{}.csv",
                self.selected_session_idx + 1
            ))?;

            std::fs::remove_file(path)?;
            self.session = Session::default();
            self.available_sessions[self.selected_session_idx] = false;

            self.confirmation = None;
        } else if self.available_sessions[self.selected_session_idx] {
            self.confirmation = Some(Confirmation::Session);
        }

        Ok(())
    }

    pub fn next_session(&mut self) -> Result<()> {
        self.selected_session_idx += 1;

        if self.selected_session_idx == self.available_sessions.len() {
            self.available_sessions.push(false);
            self.session = Session::default();
        } else if self.available_sessions[self.selected_session_idx] {
            self.load_session()?;
        } else {
            self.session = Session::default();
        }

        Ok(())
    }

    pub fn previous_session(&mut self) -> Result<()> {
        self.selected_session_idx = self.selected_session_idx.saturating_sub(1);

        if self.available_sessions[self.selected_session_idx] {
            self.load_session()?;
        } else {
            self.session = Session::default();
        }

        Ok(())
    }

    pub fn get_solves(&self) -> &Vec<Solve> {
        &self.session.solves
    }

    pub fn get_mut_solves(&mut self) -> &mut Vec<Solve> {
        &mut self.session.solves
    }

    pub fn scroll(&mut self, offset: usize) {
        self.solves_state.select(Some(offset));
        *self.solves_state.offset_mut() = offset;
    }

    pub fn scroll_up(&mut self) {
        let i = match self.solves_state.selected() {
            Some(i) if i != 0 => i - 1,
            _ => 0,
        };

        self.scroll(i);
    }

    pub fn scroll_down(&mut self) {
        let i = match self.solves_state.selected() {
            Some(i) if i < self.get_solves().len() - 1 => i + 1,
            _ => 0,
        };

        self.scroll(i);
    }

    pub fn get_stats(&self) -> &Stats {
        &self.session.stats
    }

    pub fn generate_scramble(&mut self) {
        self.last_scramble = Some(std::mem::replace(
            &mut self.scramble,
            Scramble::new(SCRAMBLE_LENGTH),
        ));
        self.generate_scramble_preview();
    }

    pub fn generate_scramble_preview(&mut self) {
        self.cube_preview = Cube::new();

        for r#move in &self.scramble.moves {
            self.cube_preview.apply_move(r#move);
        }
    }

    pub fn start_timer(&mut self) {
        self.timer.start();
        self.state = AppState::Solving;
    }

    pub fn stop_timer(&mut self) -> Result<()> {
        self.timer.stop();
        self.state = AppState::Idle;
        self.add_solve()?;
        self.session.update_stats_on_new();
        self.generate_scramble_preview();

        Ok(())
    }

    pub fn start_inspecting(&mut self) {
        self.inspection.start();
    }

    pub fn cancel_timer(&mut self) {
        self.timer.reset();
        self.inspection.stop();
        self.state = AppState::Idle;
    }

    pub fn add_solve(&mut self) -> Result<()> {
        let penalty = if self.inspection_enabled {
            self.inspection.penalty
        } else {
            Penalty::Ok
        };

        let solve = Solve::new(
            self.timer.result,
            penalty,
            None,
            None,
            std::mem::replace(&mut self.scramble, Scramble::new(SCRAMBLE_LENGTH)),
        );

        history::add_to_history(
            history::get_session_history_file(&format!(
                "session_{}.csv",
                self.selected_session_idx + 1
            ))?,
            &solve,
        )?;
        self.available_sessions[self.selected_session_idx] = true;

        self.get_mut_solves().push(solve);

        let avg_of_5 = get_avg(self.get_solves(), 5);
        let avg_of_12 = get_avg(self.get_solves(), 12);

        let solve = self.get_mut_solves().last_mut().unwrap();

        solve.avg_of_5 = avg_of_5;
        solve.avg_of_12 = avg_of_12;

        Ok(())
    }

    pub fn delete_last_solve(&mut self) -> Result<()> {
        if matches!(self.confirmation, Some(Confirmation::Solve)) {
            if self.get_mut_solves().pop().is_some() {
                self.session.update_stats();
            }

            self.confirmation = None;
            return self.rewrite_history_file();
        } else if !self.get_solves().is_empty() {
            self.confirmation = Some(Confirmation::Solve);
        }

        Ok(())
    }

    pub fn toggle_plus_two(&mut self) -> Result<()> {
        let Some(solve) = self.get_mut_solves().last_mut() else {
            return Ok(());
        };

        if matches!(solve.time.penalty, Penalty::PlusTwo) {
            solve.time.penalty = Penalty::Ok;
            solve.time.time -= 2000;
        } else {
            solve.time.penalty = Penalty::PlusTwo;
            solve.time.time += 2000;
        }

        let avg_of_5 = get_avg(self.get_solves(), 5);
        let avg_of_12 = get_avg(self.get_solves(), 12);

        let solve = self.get_mut_solves().last_mut().unwrap();

        solve.avg_of_5 = avg_of_5;
        solve.avg_of_12 = avg_of_12;

        self.session.update_stats();
        self.rewrite_history_file()
    }

    pub fn toggle_dnf(&mut self) -> Result<()> {
        let Some(solve) = self.get_mut_solves().last_mut() else {
            return Ok(());
        };

        solve.time.penalty = match solve.time.penalty {
            Penalty::Ok => Penalty::Dnf,
            Penalty::PlusTwo => {
                solve.time.time -= 2000;
                Penalty::Dnf
            }
            Penalty::Dnf => Penalty::Ok,
        };

        let avg_of_5 = get_avg(self.get_solves(), 5);
        let avg_of_12 = get_avg(self.get_solves(), 12);

        let solve = self.get_mut_solves().last_mut().unwrap();

        solve.avg_of_5 = avg_of_5;
        solve.avg_of_12 = avg_of_12;

        self.session.update_stats();
        self.rewrite_history_file()
    }

    fn rewrite_history_file(&self) -> Result<()> {
        history::update_history(
            history::get_session_history_file(&format!(
                "session_{}.csv",
                self.selected_session_idx + 1
            ))?,
            self.get_solves(),
        )
    }
}

#[derive(Default)]
pub struct Session {
    pub solves: Vec<Solve>,
    stats: Stats,
}

impl Session {
    fn update_stats_on_new(&mut self) {
        self.stats.update_on_new(&self.solves);
    }

    fn update_stats(&mut self) {
        self.stats.update(&self.solves);
    }
}

#[derive(Clone, Copy)]
pub enum Penalty {
    Ok,
    PlusTwo,
    Dnf,
}

impl From<u8> for Penalty {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Ok,
            1 => Self::PlusTwo,
            2 => Self::Dnf,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
pub struct Solve {
    pub time: StatEntry,
    pub avg_of_5: Option<StatEntry>,
    pub avg_of_12: Option<StatEntry>,
    pub scramble: Scramble,
    pub date: u64,
}

impl Solve {
    fn new(
        time: Duration,
        penalty: Penalty,
        avg_of_5: Option<StatEntry>,
        avg_of_12: Option<StatEntry>,
        scramble: Scramble,
    ) -> Self {
        Self {
            time: StatEntry::new(time.as_millis(), penalty),
            avg_of_5,
            avg_of_12,
            scramble,
            date: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn from_history_file(time: u128, penalty: u8, scramble: &str, date: u64) -> Self {
        let time = StatEntry::new(time, penalty.into());
        let scramble: Scramble = scramble.into();

        Self {
            time,
            avg_of_5: None,
            avg_of_12: None,
            scramble,
            date,
        }
    }
}
