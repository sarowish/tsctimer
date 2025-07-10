use crate::app::Penalty;
use anyhow::Result;
use rodio::{Sink, Source};
use std::time::{Duration, SystemTime};

pub const INSPECTION_DURATION: u64 = 15;

pub struct Inspection {
    starting_time: Option<SystemTime>,
    pub penalty: Penalty,
    played_sound: u8,
}

impl Inspection {
    pub fn new() -> Self {
        Inspection {
            starting_time: None,
            penalty: Penalty::Ok,
            played_sound: 0,
        }
    }

    pub fn start(&mut self) {
        self.starting_time = Some(SystemTime::now());
    }

    pub fn stop(&mut self) {
        if let Some(elapsed) = self.starting_time.and_then(|time| time.elapsed().ok()) {
            self.penalty = match elapsed.as_secs() {
                ..15 => Penalty::Ok,
                15..17 => Penalty::PlusTwo,
                _ => Penalty::Dnf,
            }
        }

        self.starting_time = None;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn elapsed(&self) -> Option<u64> {
        self.starting_time
            .and_then(|time| time.elapsed().ok())
            .map(|elapsed| elapsed.as_secs())
    }

    pub fn tick(&mut self, warning: bool) -> bool {
        if !self.is_running() {
            return true;
        }

        if let Some(elapsed) = self.elapsed() {
            if elapsed < 15 {
                if warning {
                    if elapsed == 8 && self.played_sound == 0 {
                        play_sound(425.0);
                        self.played_sound = 1;
                    } else if elapsed == 12 && self.played_sound == 1 {
                        play_sound(480.0);
                        self.played_sound = 2;
                    }
                }
                return true;
            } else if elapsed < 17 {
                return true;
            }
        }

        self.stop();
        false
    }

    pub fn is_running(&self) -> bool {
        self.starting_time.is_some()
    }

    pub fn has_expired(&self) -> bool {
        matches!(self.penalty, Penalty::Dnf)
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
