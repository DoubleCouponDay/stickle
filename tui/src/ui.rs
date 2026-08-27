use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Gauge, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Wrap,
};

use crate::app::{App, Button, Kind, Row, View};
use crate::builds::State;
use crate::checks::{Check, Status};
use crate::theme::{Mode, Theme};

const SPINNER: [char; 4] = ['|', '/', '-', '\\'];
const BUILD_MINIMUM: u16 = 7;
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

    let right =
        Layout::vertical([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)]).split(panes[1]);

    if right[1].height >= BUILD_MINIMUM {
        draw_detail(frame, right[0], app, theme);
        draw_builds(frame, right[1], app, theme);
    } else {
        app.build_buttons.clear();
        draw_detail(frame, panes[1], app, theme);
    }

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
            format!(
                " Structured Text build requirements - {} ",
                app.report.platform
            ),
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
                app.scans, app.report.env.source
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

    let dark = segment_style(theme, Mode::Dark, app.hovered, app.pressed);
    let light = segment_style(theme, Mode::Light, app.hovered, app.pressed);

    let toggle = Line::from(vec![
        Span::styled("\u{2524}", theme.dimmed()),
        Span::styled(DARK_LABEL, dark),
        Span::styled("\u{2502}", theme.dimmed()),
        Span::styled(LIGHT_LABEL, light),
        Span::styled("\u{251c}", theme.dimmed()),
    ]);

    frame.render_widget(Paragraph::new(toggle), bar);
}

fn segment_style(
    theme: Theme,
    mode: Mode,
    hovered: Option<Button>,
    pressed: Option<Button>,
) -> Style {
    let button = match mode {
        Mode::Dark => Button::Dark,
        Mode::Light => Button::Light,
    };

    let over = hovered == Some(button);
    let held = over && pressed == Some(button);

    if theme.mode == mode {
        let fill = if held {
            theme.press(theme.accent)
        } else if over {
            theme.hover(theme.accent)
        } else {
            theme.accent
        };

        return Style::new()
            .fg(theme.badge_fg)
            .bg(fill)
            .add_modifier(Modifier::BOLD);
    }

    if held {
        return Style::new().fg(theme.fg).bg(theme.surface(0.30));
    }

    if over {
        return Style::new().fg(theme.fg).bg(theme.surface(0.16));
    }

    theme.dimmed()
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
    let output = app.view == View::Output;

    let title = if output {
        format!(" {} ", app.output_label)
    } else {
        " Detail ".to_string()
    };

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme.dimmed())
        .style(theme.base())
        .title(Span::styled(
            title,
            Style::new().fg(theme.fg).add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);

    let lines = if output {
        output_lines(app, theme)
    } else {
        match app.selected_check() {
            Some(check) => detail_lines(check, theme),
            None => vec![Line::from(Span::styled(
                "Select a requirement.",
                theme.dimmed(),
            ))],
        }
    };

    let rows = if output {
        lines.len()
    } else {
        wrapped_rows(&lines, inner.width)
    };

    let limit = rows.saturating_sub(inner.height as usize) as u16;

    if app.follow && output {
        app.detail_scroll = limit;
    }

    app.detail_scroll = app.detail_scroll.min(limit);

    let mut paragraph = Paragraph::new(lines)
        .block(block)
        .scroll((app.detail_scroll, 0))
        .alignment(Alignment::Left);

    if !output {
        paragraph = paragraph.wrap(Wrap { trim: false });
    }

    frame.render_widget(paragraph, area);

    let bar = area.inner(Margin {
        vertical: 1,
        horizontal: 0,
    });

    app.detail_limit = limit;
    app.detail_track = if limit == 0 || bar.width == 0 {
        Rect::ZERO
    } else {
        Rect {
            x: bar.right() - 1,
            y: bar.y,
            width: 1,
            height: bar.height,
        }
    };

    app.detail_thumb = thumb_length(bar.height, inner.height, rows);

    let over = app.hovered == Some(Button::Scrollbar);
    let held = app.pressed == Some(Button::Scrollbar);

    let thumb = if limit == 0 {
        theme.dim
    } else if held {
        theme.press(theme.accent)
    } else if over {
        theme.hover(theme.accent)
    } else {
        theme.accent
    };

    let mut state = ScrollbarState::new(limit as usize + 1)
        .viewport_content_length(inner.height as usize)
        .position(app.detail_scroll as usize);

    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_style(if over || held {
                Style::new().fg(theme.surface(0.45))
            } else {
                theme.dimmed()
            })
            .thumb_style(Style::new().fg(thumb)),
        bar,
        &mut state,
    );

    app.detail_area = inner;
    app.detail_rows = snapshot(frame, inner);

    paint_selection(frame, app, theme);
}

fn thumb_length(track: u16, viewport: u16, rows: usize) -> u16 {
    if track == 0 || rows == 0 {
        return track;
    }

    let track = u32::from(track);
    let span = rows.max(viewport as usize) as u32;
    let length = (u32::from(viewport) * track + span / 2) / span;

    (length as u16).clamp(1, track as u16)
}

fn output_lines(app: &App, theme: Theme) -> Vec<Line<'static>> {
    if app.output.is_empty() {
        return vec![Line::from(Span::styled("Waiting for output...", theme.dimmed()))];
    }

    app.output
        .iter()
        .map(|printed| {
            Line::from(Span::styled(
                printed.text.clone(),
                kind_style(printed.kind, theme),
            ))
        })
        .collect()
}

fn kind_style(kind: Kind, theme: Theme) -> Style {
    match kind {
        Kind::Information => Style::new().fg(theme.fg),
        Kind::Pass => Style::new().fg(theme.pass),
        Kind::Warning => Style::new().fg(theme.warn),
        Kind::BuildError => Style::new().fg(theme.fail).add_modifier(Modifier::BOLD),
        Kind::RuntimeError => Style::new().fg(theme.runtime).add_modifier(Modifier::BOLD),
    }
}

fn wrapped_rows(lines: &[Line], width: u16) -> usize {
    if width == 0 {
        return lines.len();
    }

    lines
        .iter()
        .map(|line| {
            let text: String = line
                .spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect();

            wrap_count(&text, width as usize)
        })
        .sum()
}

fn wrap_count(text: &str, width: usize) -> usize {
    let mut rows = 1;
    let mut used = 0;

    for word in text.split_inclusive(' ') {
        let length = word.chars().count();

        if used + length > width && used > 0 {
            rows += 1;
            used = 0;
        }

        if length > width {
            let extra = (length - 1) / width;
            rows += extra;
            used = length - extra * width;
        } else {
            used += length;
        }
    }

    rows
}

fn draw_builds(frame: &mut Frame, area: Rect, app: &mut App, theme: Theme) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(theme.dimmed())
        .style(theme.base())
        .title(Span::styled(
            " Build ",
            Style::new().fg(theme.fg).add_modifier(Modifier::BOLD),
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let count = app.targets.len() as u16;
    let face = app
        .targets
        .iter()
        .map(|target| target.label.len() as u16 + 4)
        .max()
        .unwrap_or(0)
        .min(inner.width);

    let spacing = if inner.height >= count * 2 { 2 } else { 1 };
    let column = inner.x + (inner.width - face) / 2;

    app.build_buttons.clear();

    for index in 0..app.targets.len() {
        let y = inner.y + index as u16 * spacing;

        if y >= inner.bottom() {
            app.build_buttons.push(Rect::ZERO);
            continue;
        }

        let enabled = app.build_enabled(index);
        let state = app.build_states[index];

        let colour = match state {
            State::Running => theme.warn,
            State::Done { ok: true, .. } => theme.pass,
            State::Done { ok: false, .. } => theme.fail,
            State::Idle if enabled => theme.accent,
            State::Idle => theme.dim,
        };

        let lit = enabled || matches!(state, State::Running);
        let hovered = enabled && app.hovered == Some(Button::Build(index));
        let held = hovered && app.pressed == Some(Button::Build(index));

        let colour = if held {
            theme.press(colour)
        } else if hovered {
            theme.hover(colour)
        } else {
            colour
        };

        let button = Rect {
            x: column,
            y,
            width: face,
            height: 1,
        };

        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                centred(app.targets[index].label, face),
                if lit {
                    Style::new()
                        .fg(theme.badge_fg)
                        .bg(colour)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::new().fg(theme.dim)
                },
            )))
            .style(theme.base()),
            button,
        );

        let status = status_text(state);

        if !status.is_empty() && button.right() < inner.right() {
            let rest = Rect {
                x: button.right(),
                y,
                width: inner.right() - button.right(),
                height: 1,
            };

            frame.render_widget(
                Paragraph::new(Line::from(Span::styled(
                    format!("{status} "),
                    Style::new().fg(if lit { colour } else { theme.dim }),
                )))
                .alignment(Alignment::Right)
                .style(theme.base()),
                rest,
            );
        }

        app.build_buttons
            .push(if enabled { button } else { Rect::ZERO });
    }

    let last = inner.y + (count.saturating_sub(1)) * spacing;

    if inner.bottom() > last + 1 {
        let (text, style) = match app.build_hint() {
            Some(hint) => (hint, Style::new().fg(theme.fail)),
            None => (app.build_message.clone(), theme.dimmed()),
        };

        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(text, style))).style(theme.base()),
            Rect {
                x: inner.x,
                y: inner.bottom() - 1,
                width: inner.width,
                height: 1,
            },
        );
    }
}

fn centred(label: &str, width: u16) -> String {
    let padding = (width as usize).saturating_sub(label.len());
    let left = padding / 2;

    format!(
        "{}{label}{}",
        " ".repeat(left),
        " ".repeat(padding - left)
    )
}

fn status_text(state: State) -> String {
    match state {
        State::Idle => String::new(),
        State::Running => "running".into(),
        State::Done { ok: true, seconds, .. } => format!("built in {seconds:.1}s"),
        State::Done {
            ok: false,
            code: Some(code),
            ..
        } => format!("failed, exit {code}"),
        State::Done { ok: false, .. } => "failed".into(),
    }
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
        key("j/k", theme),
        Span::raw(" move  "),
        key("n", theme),
        Span::raw(" next unmet  "),
        key("up/down pgup/pgdn", theme),
        Span::raw(" scroll  "),
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
