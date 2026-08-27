mod app;
mod builds;
mod checks;
mod clipboard;
mod env;
mod probe;
mod scanner;
mod theme;
mod ui;

use std::io;
use std::time::Duration;

use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use ratatui::crossterm::execute;

use app::App;

const POLL: Duration = Duration::from_millis(120);

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let _ = execute!(io::stdout(), EnableMouseCapture);

    let mut app = App::new();
    let result = run(&mut terminal, &mut app);

    let _ = execute!(io::stdout(), DisableMouseCapture);
    ratatui::restore();
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> io::Result<()> {
    while !app.quit {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if event::poll(POLL)? {
            loop {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => handle_key(app, key),
                    Event::Mouse(mouse) => handle_mouse(app, mouse),
                    _ => {}
                }

                if app.quit || !event::poll(Duration::ZERO)? {
                    break;
                }
            }
        }

        app.tick();
    }

    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.quit = true;
        return;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.quit = true,
        KeyCode::Char('r') => app.rescan(),
        KeyCode::Char('t') => app.toggle_theme(),
        KeyCode::Char('n') => app.jump_to_next_unmet(),
        KeyCode::Down | KeyCode::Char('j') => app.move_selection(true),
        KeyCode::Up | KeyCode::Char('k') => app.move_selection(false),
        KeyCode::PageDown => app.scroll_detail(5),
        KeyCode::PageUp => app.scroll_detail(-5),
        KeyCode::Home => app.reset_detail_scroll(),
        _ => {}
    }
}

fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => app.mouse_down(mouse.column, mouse.row),
        MouseEventKind::Drag(MouseButton::Left) => app.mouse_drag(mouse.column, mouse.row),
        MouseEventKind::Up(MouseButton::Left) => app.mouse_up(mouse.column, mouse.row),
        MouseEventKind::Moved => app.mouse_move(mouse.column, mouse.row),
        MouseEventKind::ScrollDown => app.scroll_detail(1),
        MouseEventKind::ScrollUp => app.scroll_detail(-1),
        _ => {}
    }
}
