use crate::{
    cube::Cube,
    inspection::Inspection,
    scramble::Scramble,
    stats::{get_avg, Stats},
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
    pub solves: Vec<Solve>,
    pub stats: Stats,
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
            solves: Vec::new(),
            stats: Stats::default(),
            cube_preview: Cube::new(),
            state: AppState::Idle,
            inspection_enabled: true,
        };

        app.generate_scramble_preview();

        app
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
        self.stats.update(&self.solves);
        self.generate_scramble_preview()
    }

    pub fn start_inspecting(&mut self) {
        self.inspection.start();
    }

    pub fn toggle_inspection(&mut self) {
        self.inspection_enabled = !self.inspection_enabled;
    }

    fn add_solve(&mut self) {
        let mut times = self
            .solves
            .iter()
            .map(|solve| solve.time.as_millis())
            .collect::<Vec<u128>>();
        times.push(self.timer.result.as_millis());

        let solve = Solve::new(
            self.timer.result,
            get_avg(&times, 5),
            get_avg(&times, 12),
            std::mem::replace(&mut self.scramble, Scramble::new(SCRAMBLE_LENGTH)),
        );

        self.solves.push(solve);
    }
}

pub struct Solve {
    pub time: Duration,
    pub avg_of_5: Option<u128>,
    pub avg_of_12: Option<u128>,
    scramble: Scramble,
}

impl Solve {
    fn new(
        time: Duration,
        avg_of_5: Option<u128>,
        avg_of_12: Option<u128>,
        scramble: Scramble,
    ) -> Self {
        Self {
            time,
            avg_of_5,
            avg_of_12,
            scramble,
        }
    }
}
