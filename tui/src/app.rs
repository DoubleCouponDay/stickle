use std::collections::HashMap;
use std::time::{Duration, Instant};

use ratatui::layout::{Position, Rect};
use ratatui::widgets::ListState;

use crate::checks::{Check, Report, Status};
use crate::scanner::Scanner;
use crate::theme::{Mode, Theme};

pub const SCAN_INTERVAL: Duration = Duration::from_millis(1500);
pub const CHANGE_LIFETIME: Duration = Duration::from_secs(8);

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
            scanner: Scanner::spawn(SCAN_INTERVAL),
            statuses: HashMap::new(),
            selected: None,
        }
    }

    pub fn tick(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        self.changes
            .retain(|change| change.at.elapsed() < CHANGE_LIFETIME);

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

    pub fn click(&mut self, column: u16, row: u16) {
        let position = Position::new(column, row);

        if self.dark_button.contains(position) {
            self.theme = Theme::of(Mode::Dark);
        } else if self.light_button.contains(position) {
            self.theme = Theme::of(Mode::Light);
        }
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
                self.remember_selection();
                return;
            }
        }
    }

    pub fn scroll_detail(&mut self, delta: i16) {
        self.detail_scroll = self.detail_scroll.saturating_add_signed(delta);
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
