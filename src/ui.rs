use crate::{
    app::{App, AppState, Confirmation},
    cube::Face,
    inspection::INSPECTION_DURATION,
    stats::stat_line_to_row,
    timer::millis_to_string_not_running,
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    if app.inspection.is_running() {
        render_inspection(f, app, f.area());
        return;
    } else if matches!(app.state, AppState::Set) || app.timer.is_running() {
        render_timer(f, app, f.area());
        return;
    }

    let (left_pane_area, main_area) = if f.area().width > 140 {
        let chunks = Layout::default()
            .constraints([Constraint::Length(45), Constraint::Min(1)].as_ref())
            .direction(Direction::Horizontal)
            .split(f.area());

        (Some(chunks[0]), chunks[1])
    } else {
        (None, f.area())
    };

    let mut chunks = Layout::default()
        .constraints([Constraint::Max(3), Constraint::Min(3), Constraint::Max(13)].as_ref())
        .direction(Direction::Vertical)
        .split(main_area);

    render_scramble(f, app, chunks[0]);
    render_timer(f, app, chunks[1]);

    if let Some(area) = left_pane_area {
        render_left_pane(f, app, area);
        chunks = Layout::default()
            .constraints([
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Min(1),
                Constraint::Length(29),
            ])
            .direction(Direction::Horizontal)
            .split(chunks[2]);
    } else {
        chunks = Layout::default()
            .constraints([
                Constraint::Length(32),
                Constraint::Length(40),
                Constraint::Min(1),
                Constraint::Length(29),
            ])
            .direction(Direction::Horizontal)
            .split(chunks[2]);
        render_stats(f, app, chunks[0]);
        render_solves(f, app, chunks[1]);
    }
    render_cube(f, app, chunks[3]);

    match app.confirmation {
        Some(Confirmation::Solve) => {
            let solve_idx = app.session.selected_idx().unwrap() + 1;
            render_confirmation_window(
                f,
                &format!("Are you sure you want to delete the solve `{solve_idx}`?"),
            );
        }
        Some(Confirmation::Session) => render_confirmation_window(
            f,
            &format!(
                "Are you sure you want to delete 'Session {}'?",
                app.selected_session_idx + 1
            ),
        ),
        None => (),
    }
}

fn render_left_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Max(9), Constraint::Min(1)].as_ref())
        .direction(Direction::Vertical)
        .split(area);

    render_stats(f, app, chunks[0]);
    render_solves(f, app, chunks[1]);
}

fn render_stats(f: &mut Frame, app: &App, area: Rect) {
    let stats = vec![
        stat_line_to_row("time:", &app.get_stats().time),
        stat_line_to_row("mo3:", &app.get_stats().mean_of_3),
        stat_line_to_row("avg5:", &app.get_stats().avg_of_5),
        stat_line_to_row("avg12:", &app.get_stats().avg_of_12),
        Row::new(vec![Span::raw("")]),
        Row::new(vec![
            Span::raw("mean:"),
            Span::raw(millis_to_string_not_running(app.get_stats().global_mean)),
        ]),
    ];

    let widths = [
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ];

    let stats = Table::new(stats, widths)
        .header(
            Row::new(vec!["      ", "Current", "Best"]).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .column_spacing(2)
        .block(
            Block::default().borders(Borders::ALL).title(Span::styled(
                format!("Stats [Session {}]", app.selected_session_idx + 1),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
        );

    f.render_widget(stats, area);
}

fn render_solves(f: &mut Frame, app: &mut App, area: Rect) {
    let solves = app
        .get_solves()
        .iter()
        .enumerate()
        .rev()
        .map(|(idx, solve)| {
            vec![
                Span::raw(format!("{}.", idx + 1)),
                Span::raw(solve.time.to_string()),
                Span::raw(
                    solve
                        .avg_of_5
                        .map_or("-".to_string(), |stat| stat.to_string()),
                ),
                Span::raw(
                    solve
                        .avg_of_12
                        .map_or("-".to_string(), |stat| stat.to_string()),
                ),
            ]
        })
        .map(Row::new)
        .collect::<Vec<Row>>();

    let widths = [
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];

    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        format!(
            "Solves [{}/{}]",
            app.get_stats().valid_solve_count,
            app.get_stats().solve_count
        ),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    app.session.available_height = block.inner(area).height - 1;

    let solves = Table::new(solves, widths)
        .row_highlight_style(
            Style::new()
                .fg(Color::Magenta)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .header(
            Row::new(vec![" ", "time", "ao5", "ao12"]).style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .block(block);

    f.render_stateful_widget(solves, area, &mut app.session.state);
}

fn render_scramble(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Scramble",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));
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

fn render_timer(f: &mut Frame, app: &App, area: Rect) {
    let time = app.timer.to_string();

    let time = generate_font(&time);
    let area = center_vertically(&time, area);

    let time_text = Paragraph::new(Text::styled(
        time,
        Style::default().fg(match app.state {
            AppState::Ready if app.supports_keyboard_enhancement => Color::Red,
            AppState::Set => Color::Green,
            _ => Color::White,
        }),
    ))
    .alignment(Alignment::Center);

    f.render_widget(time_text, area);
}

fn render_inspection(f: &mut Frame, app: &App, area: Rect) {
    let Some(remaining) = app
        .inspection
        .elapsed()
        .map(|elapsed| INSPECTION_DURATION.saturating_sub(elapsed))
    else {
        return;
    };

    let time = if remaining == 0 {
        generate_font("+2")
    } else {
        generate_font(&remaining.to_string())
    };
    let area = center_vertically(&time, area);

    let time_text = Paragraph::new(Text::styled(
        time,
        Style::default().fg(match app.state {
            AppState::Ready if app.supports_keyboard_enhancement => Color::Blue,
            AppState::Idle | AppState::Ready if remaining <= 3 => Color::Red,
            AppState::Idle | AppState::Ready if remaining <= 7 => Color::Yellow,
            AppState::Set => Color::Green,
            _ => Color::White,
        }),
    ))
    .alignment(Alignment::Center);

    f.render_widget(time_text, area);
}

impl From<&Face> for Span<'_> {
    fn from(face: &Face) -> Self {
        let color: Color = (*face).into();

        Span::styled("██", Style::default().fg(color))
    }
}

fn render_cube(f: &mut Frame, app: &mut App, area: Rect) {
    let mut grid = (0..11)
        .map(|_| vec![Span::raw(" "); 15])
        .collect::<Vec<Vec<Span>>>();

    for i in 0..3 {
        grid[i][7] = Span::from(&app.cube_preview.facelets[i * 3]);
        grid[i][8] = Span::from(&app.cube_preview.facelets[i * 3 + 1]);
        grid[i][9] = Span::from(&app.cube_preview.facelets[i * 3 + 2]);

        grid[i + 4][0] = Span::from(&app.cube_preview.facelets[i * 3 + 9]);
        grid[i + 4][1] = Span::from(&app.cube_preview.facelets[i * 3 + 10]);
        grid[i + 4][2] = Span::from(&app.cube_preview.facelets[i * 3 + 11]);

        grid[i + 4][4] = Span::from(&app.cube_preview.facelets[i * 3 + 18]);
        grid[i + 4][5] = Span::from(&app.cube_preview.facelets[i * 3 + 19]);
        grid[i + 4][6] = Span::from(&app.cube_preview.facelets[i * 3 + 20]);

        grid[i + 4][8] = Span::from(&app.cube_preview.facelets[i * 3 + 27]);
        grid[i + 4][9] = Span::from(&app.cube_preview.facelets[i * 3 + 28]);
        grid[i + 4][10] = Span::from(&app.cube_preview.facelets[i * 3 + 29]);

        grid[i + 4][12] = Span::from(&app.cube_preview.facelets[i * 3 + 36]);
        grid[i + 4][13] = Span::from(&app.cube_preview.facelets[i * 3 + 37]);
        grid[i + 4][14] = Span::from(&app.cube_preview.facelets[i * 3 + 38]);

        grid[i + 8][7] = Span::from(&app.cube_preview.facelets[i * 3 + 45]);
        grid[i + 8][8] = Span::from(&app.cube_preview.facelets[i * 3 + 46]);
        grid[i + 8][9] = Span::from(&app.cube_preview.facelets[i * 3 + 47]);
    }

    let grid = grid.into_iter().map(Line::from).collect::<Vec<Line>>();
    let text = Paragraph::new(grid).block(
        Block::default().borders(Borders::ALL).title(Span::styled(
            "Scramble Preview",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
    );

    f.render_widget(text, area);
}

fn render_confirmation_window(f: &mut Frame, text: &str) {
    let window = popup_window_from_percentage(50, 15, f.area());
    f.render_widget(Clear, window);
    f.render_widget(Block::default().borders(Borders::ALL), window);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Min(1)])
        .margin(1)
        .split(window);

    let (yes_area, no_area) = {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);
        (chunks[0], chunks[1])
    };

    let mut text = Paragraph::new(Line::from(text)).alignment(Alignment::Center);
    // program crashes if width is 0 and wrap is enabled
    if chunks[0].width > 0 {
        text = text.wrap(Wrap { trim: true });
    }

    let yes = Paragraph::new(Line::from(vec![
        Span::styled("Y", Style::default().fg(Color::Green)),
        Span::raw("es"),
    ]))
    .alignment(Alignment::Center);
    let no = Paragraph::new(Line::from(vec![
        Span::styled("N", Style::default().fg(Color::Red)),
        Span::raw("o"),
    ]))
    .alignment(Alignment::Center);

    f.render_widget(text, chunks[0]);
    f.render_widget(yes, yes_area);
    f.render_widget(no, no_area);
}

fn popup_window_from_percentage(hor_percent: u16, ver_percent: u16, r: Rect) -> Rect {
    let ver = [
        Constraint::Percentage((100 - ver_percent) / 2),
        Constraint::Percentage(ver_percent),
        Constraint::Percentage((100 - ver_percent) / 2),
    ];

    let hor = [
        Constraint::Percentage((100 - hor_percent) / 2),
        Constraint::Percentage(hor_percent),
        Constraint::Percentage((100 - hor_percent) / 2),
    ];

    popup_window(&hor, &ver, r)
}

fn popup_window(hor_constraints: &[Constraint], ver_constraints: &[Constraint], r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(ver_constraints)
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(hor_constraints)
        .split(popup_layout[1])[1]
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
        } else if ch == '+' {
            convert_digit_to_font(10, &mut result);
        } else if ch == ':' {
            colon_to_font(&mut result);
        } else {
            dot_to_font(&mut result);
        }
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

fn colon_to_font(result: &mut [String]) {
    for line in result.iter_mut().take(4) {
        line.push_str("     ");
    }

    result[4].push_str("███  ");
    result[5].push_str("███  ");
    result[6].push_str("     ");
    result[7].push_str("     ");
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
    [0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0], /* + */
];
