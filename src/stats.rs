use crate::{app::Solve, timer::millis_to_string_not_running};
use ratatui::{
    style::{Color, Style},
    text::{Span, Spans},
};

#[derive(Default)]
pub struct StatEntry(pub Option<u128>, pub Option<u128>);

impl StatEntry {
    fn update(&mut self, new: Option<u128>) {
        if let Some(new) = new {
            self.0 = Some(new);
            self.1 = if let Some(time) = self.1 {
                Some(time.min(new))
            } else {
                Some(new)
            };
        }
    }
}

#[derive(Default)]
pub struct Stats {
    pub time: StatEntry,
    pub mean_of_3: StatEntry,
    pub avg_of_5: StatEntry,
    pub avg_of_12: StatEntry,
}

impl Stats {
    pub fn update(&mut self, solves: &[Solve]) {
        let times = solves
            .iter()
            .map(|solve| solve.time.as_millis())
            .collect::<Vec<u128>>();

        self.time
            .update(solves.last().map(|solve| solve.time.as_millis()));
        self.mean_of_3.update(get_mean(&times, 3));
        self.avg_of_5.update(get_avg(&times, 5));
        self.avg_of_12.update(get_avg(&times, 12));
    }
}

fn get_mean(times: &[u128], mean_of: usize) -> Option<u128> {
    get_times_from_tail(times, mean_of).map(|times| times.iter().sum::<u128>() / 3)
}

fn get_avg(times: &[u128], avg_of: usize) -> Option<u128> {
    let Some(times) = get_times_from_tail(times, avg_of) else {
        return None;
    };
    let mut total: u128 = times.iter().sum();

    total -= times.iter().min().unwrap();
    total -= times.iter().max().unwrap();

    Some(total / (avg_of - 2) as u128)
}

fn get_times_from_tail(times: &[u128], count: usize) -> Option<Vec<u128>> {
    if times.len() < count {
        None
    } else {
        Some(
            times
                .iter()
                .rev()
                .take(count)
                .map(|n| n.to_owned())
                .collect::<Vec<u128>>(),
        )
    }
}

pub fn stat_entry_to_span<'a>(s: &'a str, stat_entry: &'a StatEntry) -> Spans<'a> {
    let current = stat_entry
        .0
        .map_or("-".to_string(), millis_to_string_not_running);
    let pb = stat_entry
        .1
        .map_or("-".to_string(), millis_to_string_not_running);

    let pb = if matches!(stat_entry.1, Some(pb) if pb == stat_entry.0.unwrap()) {
        Span::styled(pb, Style::default().fg(Color::Red))
    } else {
        Span::raw(pb)
    };

    Spans::from(vec![
        Span::raw(s.to_string()),
        Span::raw(current),
        Span::raw(" | "),
        pb,
    ])
}
