use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Gauge, List, ListItem, Paragraph, Wrap};

use crate::app::{App, Row};
use crate::checks::{Check, Status};
use crate::theme::{Mode, Theme};

const SPINNER: [char; 4] = ['|', '/', '-', '\\'];
const DARK_LABEL: &str = " Dark ";
const LIGHT_LABEL: &str = " Light ";

pub fn draw(frame: &mut Frame, app: &mut App) {
    let theme = app.theme;

    frame.render_widget(Block::new().style(theme.base()), frame.area());

    let areas = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .split(frame.area());

    draw_summary(frame, areas[0], app, theme);

    let panes =
        Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)]).split(areas[1]);

    draw_list(frame, panes[0], app, theme);
    draw_detail(frame, panes[1], app, theme);
    draw_footer(frame, areas[2], theme);
    draw_theme_toggle(frame, areas[0], app, theme);
}

fn draw_summary(frame: &mut Frame, area: Rect, app: &App, theme: Theme) {
    let (total, pass, warn, fail) = app.report.totals();

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme.dimmed())
        .style(theme.base())
        .title(Span::styled(
            format!(" Stickle build requirements - {} ", app.report.platform),
            Style::new().fg(theme.fg).add_modifier(Modifier::BOLD),
        ))
        .title_bottom(
            Line::from(Span::styled(
                format!(" {} ", app.report.root.display()),
                theme.dimmed(),
            ))
            .right_aligned(),
        );

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(inner);

    let banner = if total == 0 {
        banner_span("  SCANNING  ".into(), theme.accent, theme)
    } else if fail > 0 {
        banner_span(
            format!("  BUILD BLOCKED - {fail} requirement(s) not met  "),
            theme.fail,
            theme,
        )
    } else if warn > 0 {
        banner_span(
            format!("  BUILD READY - {warn} warning(s)  "),
            theme.warn,
            theme,
        )
    } else {
        banner_span("  BUILD READY  ".into(), theme.pass, theme)
    };

    let counts = Line::from(vec![
        banner,
        Span::raw("   "),
        Span::styled(format!("{pass} met"), Style::new().fg(theme.pass)),
        Span::raw("   "),
        Span::styled(format!("{warn} warning"), Style::new().fg(theme.warn)),
        Span::raw("   "),
        Span::styled(format!("{fail} unmet"), Style::new().fg(theme.fail)),
    ]);

    frame.render_widget(Paragraph::new(counts).style(theme.base()), rows[0]);

    let ratio = if total == 0 {
        0.0
    } else {
        pass as f64 / total as f64
    };

    let filled = if total == 0 {
        theme.accent
    } else if fail > 0 {
        theme.fail
    } else if warn > 0 {
        theme.warn
    } else {
        theme.pass
    };

    let gauge = Gauge::default()
        .gauge_style(Style::new().fg(filled).bg(theme.bg))
        .ratio(ratio)
        .label(format!("{pass}/{total} requirements met"));

    frame.render_widget(gauge, rows[1]);
    frame.render_widget(Paragraph::new(activity(app, theme)).style(theme.base()), rows[2]);
}

fn banner_span(text: String, colour: Color, theme: Theme) -> Span<'static> {
    Span::styled(
        text,
        Style::new()
            .fg(theme.badge_fg)
            .bg(colour)
            .add_modifier(Modifier::BOLD),
    )
}

fn activity(app: &App, theme: Theme) -> Line<'static> {
    let spinner = SPINNER[(app.frame / 2) % SPINNER.len()];

    let age = match app.last_scan {
        Some(instant) => format!("{:.1}s ago", instant.elapsed().as_secs_f64()),
        None => "never".into(),
    };

    let mut spans = vec![
        Span::styled(
            format!("{spinner} watching"),
            Style::new().fg(theme.accent).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(
                "   scan {}   checked {age}   environment from {}",
                app.scans, app.report.env_source
            ),
            theme.dimmed(),
        ),
    ];

    if let Some((notice, _)) = app.notice.as_ref() {
        spans.push(Span::raw("   "));
        spans.push(Span::styled(
            notice.clone(),
            Style::new().fg(theme.accent).add_modifier(Modifier::BOLD),
        ));
    }

    if let Some(change) = app.changes.last() {
        spans.push(Span::raw("   "));
        spans.push(Span::styled(
            format!(
                "{} {} -> {}",
                change.label,
                change.from.badge().trim(),
                change.to.badge().trim()
            ),
            Style::new().fg(theme.change).add_modifier(Modifier::BOLD),
        ));
    }

    Line::from(spans)
}

fn draw_theme_toggle(frame: &mut Frame, area: Rect, app: &mut App, theme: Theme) {
    let width = DARK_LABEL.len() as u16 + LIGHT_LABEL.len() as u16 + 3;

    if area.width < width + 4 {
        app.dark_button = Rect::ZERO;
        app.light_button = Rect::ZERO;
        return;
    }

    let bar = Rect {
        x: area.right() - width - 2,
        y: area.y,
        width,
        height: 1,
    };

    app.dark_button = Rect {
        x: bar.x + 1,
        y: bar.y,
        width: DARK_LABEL.len() as u16,
        height: 1,
    };

    app.light_button = Rect {
        x: app.dark_button.right() + 1,
        y: bar.y,
        width: LIGHT_LABEL.len() as u16,
        height: 1,
    };

    let toggle = Line::from(vec![
        Span::styled("\u{2524}", theme.dimmed()),
        Span::styled(DARK_LABEL, segment_style(theme, Mode::Dark)),
        Span::styled("\u{2502}", theme.dimmed()),
        Span::styled(LIGHT_LABEL, segment_style(theme, Mode::Light)),
        Span::styled("\u{251c}", theme.dimmed()),
    ]);

    frame.render_widget(Paragraph::new(toggle), bar);
}

fn segment_style(theme: Theme, mode: Mode) -> Style {
    if theme.mode == mode {
        Style::new()
            .fg(theme.badge_fg)
            .bg(theme.accent)
            .add_modifier(Modifier::BOLD)
    } else {
        theme.dimmed()
    }
}

fn draw_list(frame: &mut Frame, area: Rect, app: &mut App, theme: Theme) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme.dimmed())
        .style(theme.base())
        .title(Span::styled(
            " Requirements ",
            Style::new().fg(theme.fg).add_modifier(Modifier::BOLD),
        ));

    app.list_area = block.inner(area);

    if app.rows.is_empty() {
        let waiting = Paragraph::new(vec![
            Line::raw(""),
            Line::from(Span::styled("  Scanning the environment...", theme.dimmed())),
        ])
        .block(block);

        frame.render_widget(waiting, area);
        return;
    }

    let items: Vec<ListItem> = app
        .rows
        .iter()
        .map(|row| match row {
            Row::Header { group } => ListItem::new(Line::from(vec![
                Span::raw("  "),
                Span::styled(app.report.groups[*group].title.clone(), theme.heading()),
            ])),
            Row::Item { group, check } => {
                let recent = app.is_recent(row);
                let check = &app.report.groups[*group].checks[*check];

                ListItem::new(Line::from(vec![
                    Span::styled(check.status.badge(), theme.badge(check.status)),
                    Span::styled(
                        if recent { " * " } else { "   " },
                        Style::new().fg(theme.change).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(check.name.clone(), name_style(theme, check.status)),
                ]))
            }
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol("");

    frame.render_stateful_widget(list, area, &mut app.list);
}

fn draw_detail(frame: &mut Frame, area: Rect, app: &mut App, theme: Theme) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme.dimmed())
        .style(theme.base())
        .title(Span::styled(
            " Detail ",
            Style::new().fg(theme.fg).add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);

    let lines = match app.selected_check() {
        Some(check) => detail_lines(check, theme),
        None => vec![Line::from(Span::styled(
            "Select a requirement.",
            theme.dimmed(),
        ))],
    };

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.detail_scroll, 0))
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);

    app.detail_area = inner;
    app.detail_rows = snapshot(frame, inner);

    paint_selection(frame, app, theme);
}

fn snapshot(frame: &mut Frame, area: Rect) -> Vec<String> {
    let buffer = frame.buffer_mut();

    (area.y..area.bottom())
        .map(|y| {
            (area.x..area.right())
                .map(|x| buffer[(x, y)].symbol())
                .collect()
        })
        .collect()
}

fn paint_selection(frame: &mut Frame, app: &App, theme: Theme) {
    let Some(selection) = app.selection.as_ref() else {
        return;
    };

    let buffer = frame.buffer_mut();

    for (y, from, to) in selection.spans(app.detail_area) {
        for x in from..to {
            buffer[(x, y)].set_style(theme.selection());
        }
    }
}

fn detail_lines(check: &Check, theme: Theme) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(vec![
            Span::styled(check.status.badge(), theme.badge(check.status)),
            Span::raw(" "),
            Span::styled(check.name.clone(), Style::new().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled(
                check.status.word(),
                Style::new().fg(theme.status(check.status)),
            ),
            Span::raw(" - "),
            Span::raw(check.summary.clone()),
        ]),
        Line::raw(""),
        Line::from(Span::styled("Expected", theme.heading())),
    ];

    lines.extend(body(&check.expected));
    lines.push(Line::raw(""));
    lines.push(Line::from(Span::styled("Found", theme.heading())));
    lines.extend(body(&check.found));

    if !check.remedy.is_empty() {
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled("How to fix", theme.heading())));

        for step in &check.remedy {
            lines.push(Line::from(vec![
                Span::styled("  - ", Style::new().fg(theme.warn)),
                Span::raw(step.clone()),
            ]));
        }
    }

    lines
}

fn body(text: &str) -> Vec<Line<'static>> {
    text.lines()
        .map(|line| Line::from(format!("  {line}")))
        .collect()
}

fn draw_footer(frame: &mut Frame, area: Rect, theme: Theme) {
    let keys = Line::from(vec![
        key("up/down", theme),
        Span::raw(" move  "),
        key("n", theme),
        Span::raw(" next unmet  "),
        key("pgup/pgdn", theme),
        Span::raw(" scroll detail  "),
        key("r", theme),
        Span::raw(" re-check now  "),
        key("t", theme),
        Span::raw(" theme  "),
        key("drag detail", theme),
        Span::raw(" copy  "),
        key("q", theme),
        Span::raw(" quit"),
    ]);

    frame.render_widget(
        Paragraph::new(keys).style(Style::new().fg(theme.dim).bg(theme.bg)),
        area,
    );
}

fn key(text: &str, theme: Theme) -> Span<'static> {
    Span::styled(
        text.to_string(),
        Style::new().fg(theme.accent).add_modifier(Modifier::BOLD),
    )
}

fn name_style(theme: Theme, status: Status) -> Style {
    match status {
        Status::Pass => Style::new().fg(theme.met_name),
        Status::Warn => Style::new().fg(theme.warn),
        Status::Fail => Style::new().fg(theme.fail).add_modifier(Modifier::BOLD),
    }
}
