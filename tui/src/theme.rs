use ratatui::style::{Color, Modifier, Style};

use crate::checks::Status;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Dark,
    Light,
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub mode: Mode,
    pub bg: Color,
    pub fg: Color,
    pub dim: Color,
    pub accent: Color,
    pub badge_fg: Color,
    pub pass: Color,
    pub warn: Color,
    pub fail: Color,
    pub change: Color,
    pub met_name: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Theme {
            mode: Mode::Dark,
            bg: Color::Rgb(47, 79, 79),
            fg: Color::Rgb(232, 238, 238),
            dim: Color::Rgb(158, 178, 178),
            accent: Color::Rgb(126, 214, 222),
            badge_fg: Color::Rgb(18, 30, 30),
            pass: Color::Rgb(102, 204, 128),
            warn: Color::Rgb(232, 182, 84),
            fail: Color::Rgb(240, 116, 116),
            change: Color::Rgb(222, 152, 236),
            met_name: Color::Rgb(196, 208, 208),
        }
    }

    pub fn light() -> Self {
        Theme {
            mode: Mode::Light,
            bg: Color::Rgb(246, 246, 248),
            fg: Color::Rgb(24, 26, 31),
            dim: Color::Rgb(112, 116, 126),
            accent: Color::Rgb(0, 84, 138),
            badge_fg: Color::Rgb(250, 250, 252),
            pass: Color::Rgb(21, 112, 70),
            warn: Color::Rgb(148, 94, 0),
            fail: Color::Rgb(176, 34, 52),
            change: Color::Rgb(132, 30, 148),
            met_name: Color::Rgb(74, 78, 88),
        }
    }

    pub fn of(mode: Mode) -> Self {
        match mode {
            Mode::Dark => Theme::dark(),
            Mode::Light => Theme::light(),
        }
    }

    pub fn flipped(self) -> Self {
        match self.mode {
            Mode::Dark => Theme::light(),
            Mode::Light => Theme::dark(),
        }
    }

    pub fn base(self) -> Style {
        Style::new().fg(self.fg).bg(self.bg)
    }

    pub fn dimmed(self) -> Style {
        Style::new().fg(self.dim)
    }

    pub fn status(self, status: Status) -> Color {
        match status {
            Status::Pass => self.pass,
            Status::Warn => self.warn,
            Status::Fail => self.fail,
        }
    }

    pub fn badge(self, status: Status) -> Style {
        Style::new()
            .fg(self.badge_fg)
            .bg(self.status(status))
            .add_modifier(Modifier::BOLD)
    }

    pub fn heading(self) -> Style {
        Style::new()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }
}
