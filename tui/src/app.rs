use std::collections::HashMap;
use std::time::{Duration, Instant};

use ratatui::layout::{Position, Rect};
use ratatui::widgets::ListState;

use crate::checks::{Check, Report, Status};
use crate::clipboard;
use crate::scanner::Scanner;
use crate::theme::{Mode, Theme};

pub const SCAN_INTERVAL: Duration = Duration::from_millis(1500);
pub const CHANGE_LIFETIME: Duration = Duration::from_secs(8);
pub const NOTICE_LIFETIME: Duration = Duration::from_secs(5);

pub struct Selection {
    pub anchor: (u16, u16),
    pub cursor: (u16, u16),
    pub dragging: bool,
}

impl Selection {
    fn ends(&self) -> ((u16, u16), (u16, u16)) {
        if (self.anchor.1, self.anchor.0) <= (self.cursor.1, self.cursor.0) {
            (self.anchor, self.cursor)
        } else {
            (self.cursor, self.anchor)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.cursor
    }

    pub fn spans(&self, area: Rect) -> Vec<(u16, u16, u16)> {
        if area.is_empty() || self.is_empty() {
            return Vec::new();
        }

        let (start, end) = self.ends();
        let mut spans = Vec::new();

        for y in start.1..=end.1 {
            let from = if y == start.1 { start.0 } else { area.x };
            let to = if y == end.1 { end.0 + 1 } else { area.right() };

            if from < to {
                spans.push((y, from.max(area.x), to.min(area.right())));
            }
        }

        spans
    }
}

pub enum Row {
    Header { group: usize },
    Item { group: usize, check: usize },
}

pub struct Change {
    pub label: String,
    pub from: Status,
    pub to: Status,
    pub at: Instant,
}

pub struct App {
    pub report: Report,
    pub rows: Vec<Row>,
    pub list: ListState,
    pub detail_scroll: u16,
    pub quit: bool,
    pub scans: u64,
    pub last_scan: Option<Instant>,
    pub frame: usize,
    pub changes: Vec<Change>,
    pub theme: Theme,
    pub dark_button: Rect,
    pub light_button: Rect,
    pub list_area: Rect,
    pub detail_area: Rect,
    pub detail_rows: Vec<String>,
    pub selection: Option<Selection>,
    pub notice: Option<(String, Instant)>,
    scanner: Scanner,
    statuses: HashMap<String, Status>,
    selected: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            report: Report::pending(),
            rows: Vec::new(),
            list: ListState::default(),
            detail_scroll: 0,
            quit: false,
            scans: 0,
            last_scan: None,
            frame: 0,
            changes: Vec::new(),
            theme: Theme::dark(),
            dark_button: Rect::ZERO,
            light_button: Rect::ZERO,
            list_area: Rect::ZERO,
            detail_area: Rect::ZERO,
            detail_rows: Vec::new(),
            selection: None,
            notice: None,
            scanner: Scanner::spawn(SCAN_INTERVAL),
            statuses: HashMap::new(),
            selected: None,
        }
    }

    pub fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        self.changes
            .retain(|change| change.at.elapsed() < CHANGE_LIFETIME);

        if self
            .notice
            .as_ref()
            .is_some_and(|(_, at)| at.elapsed() >= NOTICE_LIFETIME)
        {
            self.notice = None;
        }

        let Some(report) = self.scanner.latest() else {
            return;
        };

        self.apply(report);
    }

    pub fn rescan(&self) {
        self.scanner.request();
    }

    pub fn toggle_theme(&mut self) {
        self.theme = self.theme.flipped();
    }

    pub fn mouse_down(&mut self, column: u16, row: u16) {
        let position = Position::new(column, row);

        if self.dark_button.contains(position) {
            self.theme = Theme::of(Mode::Dark);
            return;
        }

        if self.light_button.contains(position) {
            self.theme = Theme::of(Mode::Light);
            return;
        }

        if self.list_area.contains(position) {
            self.selection = None;
            self.select_row_at(row);
            return;
        }

        if self.detail_area.contains(position) {
            self.selection = Some(Selection {
                anchor: (column, row),
                cursor: (column, row),
                dragging: true,
            });
        }
    }

    pub fn mouse_drag(&mut self, column: u16, row: u16) {
        let area = self.detail_area;

        let Some(selection) = self.selection.as_mut() else {
            return;
        };

        if !selection.dragging || area.is_empty() {
            return;
        }

        selection.cursor = (
            column.clamp(area.x, area.right().saturating_sub(1)),
            row.clamp(area.y, area.bottom().saturating_sub(1)),
        );
    }

    pub fn mouse_up(&mut self) {
        let Some(selection) = self.selection.as_mut() else {
            return;
        };

        if !selection.dragging {
            return;
        }

        selection.dragging = false;

        if selection.is_empty() {
            self.selection = None;
            return;
        }

        let text = self.selected_text();

        if text.is_empty() {
            self.selection = None;
            return;
        }

        self.notice = Some(match clipboard::copy(&text) {
            Ok(()) => (
                format!("copied {} character(s) to the clipboard", text.chars().count()),
                Instant::now(),
            ),
            Err(reason) => (format!("clipboard failed: {reason}"), Instant::now()),
        });
    }

    fn select_row_at(&mut self, row: u16) {
        let offset = self.list.offset() + (row - self.list_area.y) as usize;

        if matches!(self.rows.get(offset), Some(Row::Item { .. })) {
            self.list.select(Some(offset));
            self.detail_scroll = 0;
            self.remember_selection();
        }
    }

    fn selected_text(&self) -> String {
        let Some(selection) = self.selection.as_ref() else {
            return String::new();
        };

        let area = self.detail_area;

        let lines: Vec<String> = selection
            .spans(area)
            .iter()
            .filter_map(|(y, from, to)| {
                let row = self.detail_rows.get((y - area.y) as usize)?;

                let text: String = row
                    .chars()
                    .skip((from - area.x) as usize)
                    .take((to - from) as usize)
                    .collect();

                Some(text.trim_end().to_string())
            })
            .collect();

        lines.join("\n").trim_end().to_string()
    }

    fn apply(&mut self, report: Report) {
        let statuses = collect_statuses(&report);

        if !self.statuses.is_empty() {
            for (key, status) in &statuses {
                match self.statuses.get(key) {
                    Some(previous) if previous != status => self.changes.push(Change {
                        label: key.clone(),
                        from: *previous,
                        to: *status,
                        at: Instant::now(),
                    }),
                    Some(_) => {}
                    None => {}
                }
            }
        }

        self.report = report;
        self.statuses = statuses;
        self.rows = build_rows(&self.report);
        self.scans += 1;
        self.last_scan = Some(Instant::now());

        match self.selected.clone().and_then(|key| self.row_of(&key)) {
            Some(index) => self.list.select(Some(index)),
            None => self.select_first_unmet(),
        }
    }

    pub fn key_of(&self, row: &Row) -> Option<String> {
        match row {
            Row::Item { group, check } => {
                let group = &self.report.groups[*group];
                Some(format!("{} / {}", group.title, group.checks[*check].name))
            }
            Row::Header { .. } => None,
        }
    }

    pub fn is_recent(&self, row: &Row) -> bool {
        let Some(key) = self.key_of(row) else {
            return false;
        };

        self.changes.iter().any(|change| change.label == key)
    }

    fn row_of(&self, key: &str) -> Option<usize> {
        self.rows
            .iter()
            .position(|row| self.key_of(row).as_deref() == Some(key))
    }

    fn select_first_unmet(&mut self) {
        let target = self
            .rows
            .iter()
            .position(|row| self.is_unmet(row))
            .or_else(|| {
                self.rows
                    .iter()
                    .position(|row| matches!(row, Row::Item { .. }))
            });

        self.list.select(target);
        self.remember_selection();
    }

    fn remember_selection(&mut self) {
        self.selected = self
            .list
            .selected()
            .and_then(|index| self.rows.get(index))
            .and_then(|row| self.key_of(row));
    }

    pub fn selected_check(&self) -> Option<&Check> {
        match self.rows.get(self.list.selected()?)? {
            Row::Item { group, check } => Some(&self.report.groups[*group].checks[*check]),
            Row::Header { .. } => None,
        }
    }

    pub fn move_selection(&mut self, forward: bool) {
        let Some(current) = self.list.selected() else {
            self.select_first_unmet();
            return;
        };

        if self.rows.is_empty() {
            return;
        }

        let mut index = current;

        loop {
            index = if forward {
                (index + 1) % self.rows.len()
            } else if index == 0 {
                self.rows.len() - 1
            } else {
                index - 1
            };

            if index == current {
                return;
            }

            if matches!(self.rows[index], Row::Item { .. }) {
                self.list.select(Some(index));
                self.detail_scroll = 0;
                self.selection = None;
                self.remember_selection();
                return;
            }
        }
    }

    pub fn jump_to_next_unmet(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let start = self.list.selected().unwrap_or(0);

        for offset in 1..=self.rows.len() {
            let index = (start + offset) % self.rows.len();

            if self.is_unmet(&self.rows[index]) {
                self.list.select(Some(index));
                self.detail_scroll = 0;
                self.selection = None;
                self.remember_selection();
                return;
            }
        }
    }

    pub fn scroll_detail(&mut self, delta: i16) {
        self.detail_scroll = self.detail_scroll.saturating_add_signed(delta);
        self.selection = None;
    }

    pub fn reset_detail_scroll(&mut self) {
        self.detail_scroll = 0;
        self.selection = None;
    }

    fn is_unmet(&self, row: &Row) -> bool {
        match row {
            Row::Item { group, check } => {
                self.report.groups[*group].checks[*check].status == Status::Fail
            }
            Row::Header { .. } => false,
        }
    }
}

fn collect_statuses(report: &Report) -> HashMap<String, Status> {
    let mut statuses = HashMap::new();

    for group in &report.groups {
        for check in &group.checks {
            statuses.insert(format!("{} / {}", group.title, check.name), check.status);
        }
    }

    statuses
}

fn build_rows(report: &Report) -> Vec<Row> {
    let mut rows = Vec::new();

    for (group_index, group) in report.groups.iter().enumerate() {
        rows.push(Row::Header { group: group_index });

        for check_index in 0..group.checks.len() {
            rows.push(Row::Item {
                group: group_index,
                check: check_index,
            });
        }
    }

    rows
}
