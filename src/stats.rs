use crate::{app::Solve, timer::millis_to_string_not_running};
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::Row,
};

#[derive(Default)]
pub struct StatEntry(pub Option<u128>, pub Option<u128>);

impl StatEntry {
    pub fn update(&mut self, new: Option<u128>) {
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
    pub solve_count: u128,
    pub global_mean: u128,
}

impl Stats {
    pub fn update_on_new(&mut self, solves: &[Solve]) {
        let times = solves
            .iter()
            .map(|solve| solve.time.as_millis())
            .collect::<Vec<u128>>();

        self.time
            .update(solves.last().map(|solve| solve.time.as_millis()));
        self.mean_of_3.update(get_mean(&times, 3));
        self.avg_of_5.update(get_avg(&times, 5));
        self.avg_of_12.update(get_avg(&times, 12));

        self.global_mean = (self.global_mean * self.solve_count
            + solves.last().unwrap().time.as_millis())
            / (self.solve_count + 1);

        self.solve_count += 1;
    }

    pub fn update_on_delete(&mut self, deleted_solve: Solve, solves: &[Solve]) {
        self.time.0 = solves.last().map(|solve| solve.time.as_millis());

        if matches!(self.time.1, Some(pb) if pb == deleted_solve.time.as_millis()) {
            self.time.1 = solves.iter().map(|solve| solve.time.as_millis()).min();
        }

        let times = solves
            .iter()
            .map(|solve| solve.time.as_millis())
            .collect::<Vec<u128>>();

        self.mean_of_3 = StatEntry(
            get_mean(&times, 3),
            times.windows(3).map(|w| get_mean(w, 3).unwrap()).min(),
        );

        self.global_mean = (self.global_mean * self.solve_count - deleted_solve.time.as_millis())
            .checked_div(self.solve_count - 1)
            .unwrap_or_default();

        self.solve_count -= 1;

        let Some(avg_of_5) = deleted_solve.avg_of_5 else { return ; };

        if matches!(self.avg_of_5.1, Some(pb) if pb == avg_of_5) {
            self.avg_of_5 = StatEntry(
                solves
                    .last()
                    .map(|solve| solve.avg_of_5)
                    .unwrap_or_default(),
                solves
                    .iter()
                    .map(|solve| solve.avg_of_5)
                    .filter(|solve| solve.is_some())
                    .min()
                    .unwrap_or_default(),
            );
        }

        let Some(avg_of_12) = deleted_solve.avg_of_12 else { return ; };

        if matches!(self.avg_of_12.1, Some(pb) if pb == avg_of_12) {
            self.avg_of_12 = StatEntry(
                solves
                    .last()
                    .map(|solve| solve.avg_of_12)
                    .unwrap_or_default(),
                solves.iter().filter_map(|solve| solve.avg_of_12).min(),
            );
        }
    }
}

fn get_mean(times: &[u128], mean_of: usize) -> Option<u128> {
    get_times_from_tail(times, mean_of).map(|times| times.iter().sum::<u128>() / 3)
}

pub fn get_avg(times: &[u128], avg_of: usize) -> Option<u128> {
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

pub fn stat_entry_to_row<'a>(s: &'a str, stat_entry: &'a StatEntry) -> Row<'a> {
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

    Row::new(vec![Span::raw(s.to_string()), Span::raw(current), pb])
}
