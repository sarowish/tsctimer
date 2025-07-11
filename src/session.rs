use ratatui::widgets::TableState;

use crate::{
    app::Solve,
    stats::{get_avg, get_solves_from_tail, Stats},
};

#[derive(Default)]
pub struct Session {
    pub solves: Vec<Solve>,
    pub state: TableState,
    pub available_height: u16,
    pub stats: Stats,
}

impl Session {
    pub fn update_stats_on_new(&mut self) {
        self.stats.update_on_new(&self.solves);
        self.select_first();
    }

    pub fn update_stats(&mut self) {
        self.stats.update(&self.solves);
    }

    pub fn update_around(&mut self, idx: usize) {
        let len = self.solves.len();

        for idx in idx..len.min(idx + 5) {
            self.solves[idx].avg_of_5 =
                get_solves_from_tail(&self.solves[..=idx], 5).and_then(|solves| get_avg(solves, 5));
        }

        for idx in idx..len.min(idx + 12) {
            self.solves[idx].avg_of_12 = get_solves_from_tail(&self.solves[..=idx], 12)
                .and_then(|solves| get_avg(solves, 12));
        }

        self.update_stats();
    }

    pub fn selected_idx(&self) -> Option<usize> {
        self.state.selected().map(|idx| self.solves.len() - idx - 1)
    }

    fn select_with_index(&mut self, index: usize) {
        self.state.select(if self.solves.is_empty() {
            None
        } else {
            Some(index)
        });
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.solves.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.select_with_index(i);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.solves.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.select_with_index(i);
    }

    pub fn select_first(&mut self) {
        self.select_with_index(0);
    }

    pub fn select_last(&mut self) {
        self.select_with_index(self.solves.len().checked_sub(1).unwrap_or_default());
    }

    pub fn scroll_up(&mut self, by: usize, move_cursor: bool) {
        let offset = self.state.offset_mut();
        *offset = offset.saturating_sub(by);

        let offset = self.state.offset();

        let Some(selected) = self.state.selected_mut() else {
            return;
        };

        if move_cursor {
            *selected = selected.saturating_sub(by);
        } else if *selected >= offset + self.available_height as usize {
            *selected -= 1;
        }
    }

    pub fn scroll_down(&mut self, by: usize, move_cursor: bool) {
        let len = self.solves.len();
        let offset = self.state.offset_mut();
        *offset = (*offset + by).min(len);

        let offset = self.state.offset();

        let Some(selected) = self.state.selected_mut() else {
            return;
        };

        if move_cursor {
            *selected = (*selected + by).min(len);
        } else if *selected < offset {
            *selected += 1;
        }
    }

    pub fn scroll_up_half(&mut self) {
        self.scroll_up(self.available_height as usize / 2, true);
    }

    pub fn scroll_down_half(&mut self) {
        self.scroll_down(self.available_height as usize / 2, true);
    }

    pub fn scroll_up_full(&mut self) {
        self.scroll_up(self.available_height as usize, true);
    }

    pub fn scroll_down_full(&mut self) {
        self.scroll_down(self.available_height as usize, true);
    }
}
