use crate::{
    cube::Cube,
    inspection::Inspection,
    scramble::Scramble,
    stats::{get_avg, StatEntry, Stats},
    timer::Timer,
};
use std::time::Duration;

pub const SCRAMBLE_LENGTH: u8 = 25;

pub enum AppState {
    Idle,
    Solving,
    Set,
    Ready,
}

pub struct App {
    pub timer: Timer,
    pub inspection: Inspection,
    pub scramble: Scramble,
    pub sessions: Vec<Session>,
    pub selected_session_idx: usize,
    pub cube_preview: Cube,
    pub state: AppState,
    pub inspection_enabled: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            timer: Timer::new(),
            inspection: Inspection::new(),
            scramble: Scramble::new(SCRAMBLE_LENGTH),
            sessions: vec![Session::default()],
            selected_session_idx: 0,
            cube_preview: Cube::new(),
            state: AppState::Idle,
            inspection_enabled: true,
        };

        app.generate_scramble_preview();

        app
    }

    pub fn next_session(&mut self) {
        self.selected_session_idx += 1;

        if self.selected_session_idx == self.sessions.len() {
            self.sessions.push(Session::default())
        }
    }

    pub fn previous_session(&mut self) {
        self.selected_session_idx = self.selected_session_idx.saturating_sub(1);
    }

    fn get_selected_session(&self) -> &Session {
        &self.sessions[self.selected_session_idx]
    }

    fn get_mut_selected_session(&mut self) -> &mut Session {
        &mut self.sessions[self.selected_session_idx]
    }

    pub fn get_solves(&self) -> &Vec<Solve> {
        &self.get_selected_session().solves
    }

    pub fn get_mut_solves(&mut self) -> &mut Vec<Solve> {
        &mut self.get_mut_selected_session().solves
    }

    pub fn get_stats(&self) -> &Stats {
        &self.get_selected_session().stats
    }

    pub fn generate_scramble(&mut self) {
        self.scramble = Scramble::new(SCRAMBLE_LENGTH);
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

    pub fn stop_timer(&mut self) {
        self.timer.stop();
        self.state = AppState::Idle;
        self.add_solve();
        self.get_mut_selected_session().update_stats_on_new();
        self.generate_scramble_preview()
    }

    pub fn start_inspecting(&mut self) {
        self.inspection.start();
    }

    pub fn cancel_timer(&mut self) {
        self.timer.reset();
        self.inspection.stop();
        self.state = AppState::Idle;
    }

    pub fn toggle_inspection(&mut self) {
        self.inspection_enabled = !self.inspection_enabled;
    }

    fn add_solve(&mut self) {
        let solve = Solve::new(
            self.timer.result,
            None,
            None,
            std::mem::replace(&mut self.scramble, Scramble::new(SCRAMBLE_LENGTH)),
        );

        self.get_mut_solves().push(solve);

        let avg_of_5 = get_avg(self.get_solves(), 5);
        let avg_of_12 = get_avg(self.get_solves(), 12);

        let solve = self.get_mut_solves().last_mut().unwrap();

        solve.avg_of_5 = avg_of_5;
        solve.avg_of_12 = avg_of_12;
    }

    pub fn delete_last_solve(&mut self) {
        let Some(solve) = self.get_mut_solves().pop() else { return; };
        self.get_mut_selected_session().update_stats(&solve)
    }

    pub fn toggle_plus_two(&mut self) {
        let Some(solve) = self.get_mut_solves().last_mut() else { return; };
        let prev = solve.clone();

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

        self.get_mut_selected_session().update_stats(&prev);
    }

    pub fn toggle_dnf(&mut self) {
        let Some(solve) = self.get_mut_solves().last_mut() else { return; };
        let prev = solve.clone();

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

        self.get_mut_selected_session().update_stats(&prev);
    }
}

#[derive(Default)]
pub struct Session {
    solves: Vec<Solve>,
    stats: Stats,
}

impl Session {
    fn update_stats_on_new(&mut self) {
        self.stats.update_on_new(&self.solves);
    }

    fn update_stats(&mut self, relevant_solve: &Solve) {
        self.stats.update(relevant_solve, &self.solves);
    }
}

#[derive(Clone, Copy)]
pub enum Penalty {
    Ok,
    PlusTwo,
    Dnf,
}

#[derive(Clone)]
pub struct Solve {
    pub time: StatEntry,
    pub avg_of_5: Option<StatEntry>,
    pub avg_of_12: Option<StatEntry>,
    scramble: Scramble,
}

impl Solve {
    fn new(
        time: Duration,
        avg_of_5: Option<StatEntry>,
        avg_of_12: Option<StatEntry>,
        scramble: Scramble,
    ) -> Self {
        Self {
            time: StatEntry::new(time.as_millis(), Penalty::Ok),
            avg_of_5,
            avg_of_12,
            scramble,
        }
    }
}
