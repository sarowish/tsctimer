use std::time::SystemTime;

const INSPECTING_TIME: u64 = 15;

pub struct Inspection {
    duration: u64,
    starting_time: Option<SystemTime>,
    pub expired: bool,
}

impl Inspection {
    pub fn new() -> Self {
        Inspection {
            duration: INSPECTING_TIME,
            starting_time: None,
            expired: false,
        }
    }

    pub fn start(&mut self) {
        self.starting_time = Some(SystemTime::now())
    }

    pub fn stop(&mut self) {
        self.starting_time = None;
    }

    pub fn remaining(&mut self) -> Option<u64> {
        if let Some(starting_time) = self.starting_time {
            let elapsed = starting_time.elapsed().unwrap().as_secs();

            if elapsed <= 15 {
                return Some(self.duration - elapsed);
            }
        }

        self.stop();

        None
    }

    pub fn is_running(&self) -> bool {
        self.starting_time.is_some()
    }
}
