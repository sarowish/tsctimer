use crate::{
    app::{self, App, AppState},
    scramble::Scramble,
    stats::stat_entry_to_row,
    timer::millis_to_string_not_running,
};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if app.inspection.is_running() {
        render_inspection(f, app, f.size());
        return;
    } else if matches!(app.state, AppState::Set) || app.timer.is_running() {
        render_timer(f, app, f.size());
        return;
    }

    let mut chunks = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(90)].as_ref())
        .direction(Direction::Horizontal)
        .split(f.size());

    render_left_pane(f, app, chunks[0]);

    chunks = Layout::default()
        .constraints([Constraint::Max(3), Constraint::Min(3)].as_ref())
        .direction(Direction::Vertical)
        .split(chunks[1]);

    render_scramble(f, app, chunks[0]);
    render_timer(f, app, chunks[1]);
}

fn render_left_pane<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Max(9), Constraint::Min(1)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    let stats = vec![
        stat_entry_to_row("time:", &app.stats.time),
        stat_entry_to_row("mo3:", &app.stats.mean_of_3),
        stat_entry_to_row("avg5:", &app.stats.avg_of_5),
        stat_entry_to_row("avg12:", &app.stats.avg_of_12),
        Row::new(vec![Span::raw("")]),
        Row::new(vec![
            Span::raw("session mean:"),
            Span::raw(millis_to_string_not_running(app.stats.global_mean)),
        ]),
    ];

    let stats = Table::new(stats)
        .header(Row::new(vec!["      ", "Current", "Best"]))
        .column_spacing(2)
        .widths(&[
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(stats, chunks[0]);

    let solves = app
        .solves
        .iter()
        .rev()
        .enumerate()
        .rev()
        .map(|(idx, solve)| {
            vec![
                Span::raw(format!("{}.", idx + 1)),
                Span::raw(millis_to_string_not_running(solve.time.as_millis())),
                Span::raw(
                    solve
                        .avg_of_5
                        .map_or("-".to_string(), millis_to_string_not_running),
                ),
                Span::raw(
                    solve
                        .avg_of_12
                        .map_or("-".to_string(), millis_to_string_not_running),
                ),
            ]
        })
        .map(Row::new)
        .collect::<Vec<Row>>();

    let mut state = TableState::default();

    if !solves.is_empty() {
        state.select(Some(solves.len() - 1));
    }

    let solves = Table::new(solves)
        .header(Row::new(vec![" ", "time", "ao5", "ao12"]))
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .block(Block::default().borders(Borders::ALL));

    f.render_stateful_widget(solves, chunks[1], &mut state);
}

fn render_scramble<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, area);

    let scramble = app.scramble.to_string();

    let area = center_vertically(&scramble, area);

    let scramble = Paragraph::new(Span::styled(
        scramble,
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    f.render_widget(scramble, area);
}

fn render_timer<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let time = app.timer.to_string();

    let time = generate_font(&time);
    let area = center_vertically(&time, area);

    let time_text = Paragraph::new(Text::styled(
        time,
        Style::default().fg(if matches!(app.state, AppState::Set) {
            Color::Green
        } else {
            Color::White
        }),
    ))
    .alignment(Alignment::Center);

    f.render_widget(time_text, area);
}

fn render_inspection<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let Some(remaining) = app.inspection.remaining() else {
        app.inspection.stop();
        app.inspection.expired = true;
        app.state = AppState::Idle;
        app.scramble = Scramble::new(app::SCRAMBLE_LENGTH);
        return;
    };

    let time = generate_font(&remaining.to_string());
    let area = center_vertically(&time, area);

    let time_text = Paragraph::new(Text::styled(
        time,
        Style::default().fg(match app.state {
            AppState::Idle | AppState::Ready if remaining <= 4 => Color::Red,
            AppState::Idle | AppState::Ready if remaining <= 8 => Color::Yellow,
            AppState::Set => Color::Green,
            _ => Color::White,
        }),
    ))
    .alignment(Alignment::Center);

    f.render_widget(time_text, area);
}

fn center_vertically(text: &str, area: Rect) -> Rect {
    let line_count = text.lines().count() as u16;

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(area.height.saturating_sub(line_count) / 2),
            Constraint::Length(line_count),
            Constraint::Min(1),
        ])
        .direction(Direction::Vertical)
        .split(area);

    chunks[1]
}

fn generate_font(text: &str) -> String {
    let mut result = (0..10).map(|_| String::new()).collect::<Vec<String>>();

    for ch in text.chars() {
        if let Some(digit) = ch.to_digit(10) {
            convert_digit_to_font(digit, &mut result);
        } else {
            dot_to_font(&mut result);
        };
    }

    result.join("\n")
}

fn convert_digit_to_font(digit: u32, result: &mut Vec<String>) {
    for i in 0..3 {
        for j in 0..5 {
            let c = if NUMBERS[digit as usize][j * 3 + i] == 1 {
                "████"
            } else {
                "    "
            };

            result[j * 2].push_str(c);
            result[j * 2 + 1].push_str(c);
        }
    }

    for line in result {
        line.push_str("  ");
    }
}

fn dot_to_font(result: &mut [String]) {
    for line in result.iter_mut().take(8) {
        line.push_str("     ");
    }
    result[8].push_str("███  ");
    result[9].push_str("███  ");
}

const NUMBERS: [[i32; 15]; 11] = [
    [1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1], /* 0 */
    [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1], /* 1 */
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1], /* 2 */
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1], /* 3 */
    [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1], /* 4 */
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1], /* 5 */
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1], /* 6 */
    [1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1], /* 7 */
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1], /* 8 */
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1], /* 9 */
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1], /* 9 */
];
