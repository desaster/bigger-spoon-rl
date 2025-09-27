// NOTE: This file was generated with heavy LLM assistance.
//       Treat the code as experimental and review before trusting it.

// Enforce that exactly one frontend is active; otherwise fail at compile time.
#[cfg(any(
    all(feature = "web", feature = "native"),
    not(any(feature = "web", feature = "native")),
))]
compile_error!("enable exactly one of the features `web` or `native`");

mod config;
mod game;
mod render;
mod theme;

#[cfg(feature = "native")]
use theme::Theme;

#[cfg(feature = "native")]
use crate::game::Input;
#[cfg(feature = "native")]
use crossterm::event::KeyCode;

#[cfg(feature = "web")]
mod web;

#[cfg(feature = "web")]
fn main() {
    web::run();
}

#[cfg(feature = "native")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::{io, time::Duration};

    use crossterm::{
        event::{self, Event},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    };
    use ratatui::{Terminal, backend::CrosstermBackend};

    use crate::game::{Action, Game};

    // Standard Crossterm lifecycle: enter raw + alternate screen, run the game loop,
    // then restore the terminal before exiting.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut game = Game::default();
    let theme = Theme::default();

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            render::draw(&game, area, frame, &theme);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let Some(input) = map_key_to_input(&key.code) {
                    // Game state decides whether to redraw, quit, or trigger special actions.
                    match game.handle_input(input) {
                        Action::Quit | Action::Descend => break,
                        Action::Redraw => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

#[cfg(feature = "native")]
fn map_key_to_input(code: &KeyCode) -> Option<Input> {
    // Central place for translating vim-style keys into game commands.
    match code {
        KeyCode::Char('q') | KeyCode::Esc => Some(Input::Quit),
        KeyCode::Char('h') => Some(Input::MoveLeft),
        KeyCode::Char('l') => Some(Input::MoveRight),
        KeyCode::Char('k') => Some(Input::MoveUp),
        KeyCode::Char('j') => Some(Input::MoveDown),
        KeyCode::Char('y') => Some(Input::MoveUpLeft),
        KeyCode::Char('u') => Some(Input::MoveUpRight),
        KeyCode::Char('b') => Some(Input::MoveDownLeft),
        KeyCode::Char('n') => Some(Input::MoveDownRight),
        KeyCode::Char('>') => Some(Input::Descend),
        _ => None,
    }
}
