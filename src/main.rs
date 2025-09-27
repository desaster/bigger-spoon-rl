#[cfg(any(
    all(feature = "web", feature = "native"),
    not(any(feature = "web", feature = "native")),
))]
compile_error!("enable exactly one of the features `web` or `native`");

mod game;
mod render;

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
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};

    use crate::game::{Action, Game};

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let mut game = Game::default();

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            render::draw(&game, area, frame);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let Some(input) = map_key_to_input(&key.code) {
                    if matches!(game.handle_input(input), Action::Quit) {
                        break;
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
fn map_key_to_input(code: &crossterm::event::KeyCode) -> Option<crate::game::Input> {
    match code {
        crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => {
            Some(crate::game::Input::Quit)
        }
        crossterm::event::KeyCode::Char('h') => Some(crate::game::Input::MoveLeft),
        crossterm::event::KeyCode::Char('l') => Some(crate::game::Input::MoveRight),
        crossterm::event::KeyCode::Char('k') => Some(crate::game::Input::MoveUp),
        crossterm::event::KeyCode::Char('j') => Some(crate::game::Input::MoveDown),
        crossterm::event::KeyCode::Char('y') => Some(crate::game::Input::MoveUpLeft),
        crossterm::event::KeyCode::Char('u') => Some(crate::game::Input::MoveUpRight),
        crossterm::event::KeyCode::Char('b') => Some(crate::game::Input::MoveDownLeft),
        crossterm::event::KeyCode::Char('n') => Some(crate::game::Input::MoveDownRight),
        _ => None,
    }
}
