use anyhow::Result;
use rodio::{Sink, Source};
use std::time::{Duration, SystemTime};

const INSPECTING_TIME: u64 = 15;

pub struct Inspection {
    duration: u64,
    starting_time: Option<SystemTime>,
    pub expired: bool,
    played_sound: u8,
}

impl Inspection {
    pub fn new() -> Self {
        Inspection {
            duration: INSPECTING_TIME,
            starting_time: None,
            expired: false,
            played_sound: 0,
        }
    }

    pub fn start(&mut self) {
        self.starting_time = Some(SystemTime::now())
    }

    pub fn stop(&mut self) {
        self.starting_time = None;
        self.played_sound = 0;
    }

    pub fn remaining(&mut self) -> Option<u64> {
        if let Some(starting_time) = self.starting_time {
            let elapsed = starting_time.elapsed().unwrap().as_secs();

            if elapsed <= 15 {
                if elapsed == 8 && self.played_sound == 0 {
                    play_sound(425.0);
                    self.played_sound = 1;
                } else if elapsed == 12 && self.played_sound == 1 {
                    play_sound(480.0);
                    self.played_sound = 2;
                }

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

fn play_sound(frequency: f32) {
    std::thread::spawn(move || -> Result<()> {
        let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.append(
            rodio::source::SineWave::new(frequency).take_duration(Duration::from_millis(500)),
        );
        sink.sleep_until_end();
        Ok(())
    });
}
