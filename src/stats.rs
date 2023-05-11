use std::{cmp::Ordering, fmt::Display};

use crate::{
    app::{Penalty, Solve},
    timer::millis_to_string_not_running,
};
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::Row,
};

#[derive(Clone, Copy)]
pub struct StatEntry {
    pub time: u128,
    pub penalty: Penalty,
}

impl StatEntry {
    pub fn new(time: u128, penalty: Penalty) -> Self {
        Self { time, penalty }
    }
}

impl Display for StatEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.penalty {
                Penalty::Dnf => "DNF".to_string(),
                _ => millis_to_string_not_running(self.time),
            },
            match self.penalty {
                Penalty::PlusTwo => "+",
                _ => "",
            }
        )
    }
}

impl PartialEq for StatEntry {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Eq for StatEntry {}

impl Ord for StatEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for StatEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default)]
pub struct StatLine {
    current: Option<StatEntry>,
    best: Option<StatEntry>,
}

impl StatLine {
    fn new(current: Option<StatEntry>, best: Option<StatEntry>) -> Self {
        Self { current, best }
    }

    pub fn update(&mut self, new: Option<StatEntry>) {
        if let Some(new) = new {
            self.current = Some(new);
            self.best = if let Some(best) = self.best {
                Some(best.min(new))
            } else {
                Some(new)
            };
        }
    }
}

#[derive(Default)]
pub struct Stats {
    pub time: StatLine,
    pub mean_of_3: StatLine,
    pub avg_of_5: StatLine,
    pub avg_of_12: StatLine,
    pub solve_count: u128,
    pub global_mean: u128,
}

impl Stats {
    pub fn update(&mut self, solves: &[Solve]) {
        self.time.current = solves.last().map(|solve| solve.time);
        self.time.best = solves.iter().map(|solve| solve.time).min();

        self.mean_of_3 = StatLine::new(
            get_mean(solves, 3),
            solves.windows(3).filter_map(|w| get_mean(w, 3)).min(),
        );

        self.solve_count = solves.len() as u128;

        self.global_mean = solves
            .iter()
            .map(|solve| solve.time.time)
            .sum::<u128>()
            .checked_div(self.solve_count)
            .unwrap_or_default();

        self.avg_of_5 = StatLine::new(
            solves
                .last()
                .map(|solve| solve.avg_of_5)
                .unwrap_or_default(),
            solves.iter().filter_map(|solve| solve.avg_of_5).min(),
        );

        self.avg_of_12 = StatLine::new(
            solves
                .last()
                .map(|solve| solve.avg_of_12)
                .unwrap_or_default(),
            solves.iter().filter_map(|solve| solve.avg_of_12).min(),
        );
    }

    pub fn update_on_new(&mut self, solves: &[Solve]) {
        self.time.update(solves.last().map(|solve| solve.time));
        self.mean_of_3.update(get_mean(solves, 3));
        self.avg_of_5.update(get_avg(solves, 5));
        self.avg_of_12.update(get_avg(solves, 12));

        self.global_mean = (self.global_mean * self.solve_count + solves.last().unwrap().time.time)
            / (self.solve_count + 1);

        self.solve_count += 1;
    }
}

fn get_mean(solves: &[Solve], mean_of: usize) -> Option<StatEntry> {
    let Some(solves) = get_solves_from_tail(solves, mean_of) else {
        return None;
    };

    let mut sum = 0;

    for solve in solves {
        if matches!(solve.time.penalty, Penalty::Dnf) {
            return Some(StatEntry::new(u128::MAX, Penalty::Dnf));
        }

        sum += solve.time.time;
    }

    Some(StatEntry::new(sum / 3, Penalty::Ok))
}

pub fn get_avg(solves: &[Solve], avg_of: usize) -> Option<StatEntry> {
    let Some(solves) = get_solves_from_tail(solves, avg_of) else {
        return None;
    };

    let mut sum = 0;
    let mut min = u128::MAX;
    let mut max = u128::MIN;
    let mut dnf_count = 0;

    for solve in solves {
        if matches!(solve.time.penalty, Penalty::Dnf) {
            dnf_count += 1;
            if dnf_count == 2 {
                return Some(StatEntry::new(0, Penalty::Dnf));
            }
        } else {
            sum += solve.time.time;
            min = min.min(solve.time.time);
            max = max.max(solve.time.time);
        }
    }

    if dnf_count == 0 {
        sum -= max;
    }
    sum -= min;

    Some(StatEntry::new(sum / (avg_of - 2) as u128, Penalty::Ok))
}

fn get_solves_from_tail(solves: &[Solve], count: usize) -> Option<&[Solve]> {
    let length = solves.len();

    if length < count {
        None
    } else {
        Some(&solves[length - count..length])
    }
}

pub fn stat_line_to_row<'a>(s: &'a str, stat_line: &'a StatLine) -> Row<'a> {
    let current = stat_line
        .current
        .map_or("-".to_string(), |current| current.to_string());
    let pb = stat_line
        .best
        .map_or("-".to_string(), |best| best.to_string());

    let pb = if stat_line.current.is_some() && stat_line.best == stat_line.current {
        Span::styled(pb, Style::default().fg(Color::Red))
    } else {
        Span::raw(pb)
    };

    Row::new(vec![Span::raw(s.to_string()), Span::raw(current), pb])
}
