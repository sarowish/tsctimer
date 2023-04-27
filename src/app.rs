use crate::{scramble::Scramble, stats::Stats, timer::Timer};
use std::time::Duration;

const SCRAMBLE_LENGTH: u8 = 25;

pub struct App {
    pub timer: Timer,
    pub scramble: Scramble,
    pub solves: Vec<Solve>,
    pub stats: Stats,
    pub holding_space_count: u8,
}

impl App {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            scramble: Scramble::new(SCRAMBLE_LENGTH),
            solves: Vec::new(),
            stats: Stats::default(),
            holding_space_count: 0,
        }
    }

    pub fn stop_timer(&mut self) {
        self.timer.stop();

        let solve = Solve::new(
            self.timer.result,
            std::mem::replace(&mut self.scramble, Scramble::new(SCRAMBLE_LENGTH)),
        );
        self.solves.push(solve);
        self.stats.update(&self.solves);
    }
}

pub struct Solve {
    pub time: Duration,
    scramble: Scramble,
}

impl Solve {
    fn new(time: Duration, scramble: Scramble) -> Self {
        Self { time, scramble }
    }
}
