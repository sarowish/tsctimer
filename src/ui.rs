use crate::{app::App, stats::stat_entry_to_span, timer::millis_to_string_not_running};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    if app.holding_space_count > 1 || app.timer.is_running() {
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
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    let stats = vec![
        ListItem::new(stat_entry_to_span("time: ", &app.stats.time)),
        ListItem::new(stat_entry_to_span("mo3: ", &app.stats.mean_of_3)),
        ListItem::new(stat_entry_to_span("avg5: ", &app.stats.avg_of_5)),
        ListItem::new(stat_entry_to_span("avg12: ", &app.stats.avg_of_12)),
    ];

    let stats = List::new(stats).block(Block::default().borders(Borders::ALL));

    f.render_widget(stats, chunks[0]);

    let solves = app
        .solves
        .iter()
        .enumerate()
        .map(|(idx, solve)| {
            format!(
                "{}. {}",
                idx,
                millis_to_string_not_running(solve.time.as_millis())
            )
        })
        .map(Span::raw)
        .map(ListItem::new)
        .collect::<Vec<ListItem>>();

    let solves = List::new(solves).block(Block::default().borders(Borders::ALL));

    f.render_widget(solves, chunks[1]);
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
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, area);

    let time = app.timer.to_string();

    let time = generate_font(&time);
    let area = center_vertically(&time, area);

    let time_text = Paragraph::new(Text::styled(
        time,
        Style::default().fg(if app.holding_space_count > 1 {
            Color::Green
        } else {
            Color::White
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
