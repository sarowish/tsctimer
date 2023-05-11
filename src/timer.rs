use std::{
    fmt,
    time::{Duration, SystemTime},
};

pub struct Timer {
    starting_time: Option<SystemTime>,
    pub result: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            starting_time: None,
            result: Duration::new(0, 0),
        }
    }

    pub fn start(&mut self) {
        self.starting_time = Some(SystemTime::now())
    }

    pub fn stop(&mut self) {
        if let Some(starting_time) = self.starting_time {
            self.result = starting_time.elapsed().unwrap();
        }

        self.starting_time = None;
    }

    pub fn reset(&mut self) {
        self.starting_time = None;
        self.result = Duration::new(0, 0);
    }

    pub fn is_running(&self) -> bool {
        self.starting_time.is_some()
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time = if let Some(starting_time) = self.starting_time {
            starting_time.elapsed().unwrap()
        } else {
            self.result
        }
        .as_millis();

        let time_text = millis_to_string(time, self.is_running());

        write!(f, "{}", time_text)
    }
}

pub fn millis_to_string(time: u128, is_running: bool) -> String {
    let millis = time % 1000;
    let seconds = (time / 1000) % 60;
    let minutes = time / 60000;

    match (minutes, seconds, millis) {
        (0, _, _) => {
            if is_running {
                format!("{}.{}", seconds, millis / 100)
            } else {
                format!("{}.{:03}", seconds, millis)
            }
        }
        _ => {
            if is_running {
                format!("{}:{}.{}", minutes, seconds, millis / 100)
            } else {
                format!("{}:{}.{:03}", minutes, seconds, millis)
            }
        }
    }
}

pub fn millis_to_string_not_running(time: u128) -> String {
    millis_to_string(time, false)
}
