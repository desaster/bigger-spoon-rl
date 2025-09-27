#![cfg(feature = "web")]

use base16_palettes::{
    palettes::{DefaultDark, DefaultPalette},
    Palette,
};
use ratatui::prelude::*;
use webatui::prelude::*;
use yew::{Context, Renderer};

use crate::{
    game::{Action, Game, Input},
    render,
};

impl TerminalApp for Game {
    type Message = Input;

    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
    fn setup(&mut self, ctx: &Context<WebTerminal<Self>>) {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::{
                wasm_bindgen::{prelude::Closure, JsCast},
                KeyboardEvent,
            };

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        // Mirror the in-app background (`dark_1`) so the page chrome matches.
                        let style = body.style();
                        let _ = style.set_property("background-color", "#181818");
                    }
                }

                let callback = ctx.link().callback(|msg: Input| WebTermMessage::new(msg));
                let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
                    match event.key().as_str() {
                        "q" | "Q" => callback.emit(Input::Quit),
                        "h" | "H" => callback.emit(Input::MoveLeft),
                        "l" | "L" => callback.emit(Input::MoveRight),
                        "k" | "K" => callback.emit(Input::MoveUp),
                        "j" | "J" => callback.emit(Input::MoveDown),
                        "y" | "Y" => callback.emit(Input::MoveUpLeft),
                        "u" | "U" => callback.emit(Input::MoveUpRight),
                        "b" | "B" => callback.emit(Input::MoveDownLeft),
                        "n" | "N" => callback.emit(Input::MoveDownRight),
                        _ => {}
                    }
                }) as Box<dyn FnMut(_)>);

                let _ = window.add_event_listener_with_callback(
                    "keydown",
                    closure.as_ref().unchecked_ref(),
                );

                closure.forget();
            }
        }
    }

    fn update(&mut self, _ctx: TermContext<'_, Self>, input: Self::Message) -> bool {
        match self.handle_input(input) {
            Action::Redraw => true,
            Action::Quit => false,
        }
    }

    fn render(&self, area: Rect, frame: &mut Frame<'_>) {
        render::draw(self, area, frame);
    }
}

pub fn run() {
    let palette = Palette::DefaultPalette(DefaultPalette::DefaultDark(DefaultDark));
    let props = WebTermProps::new_with_palette(Game::default(), palette);
    Renderer::<WebTerminal<Game>>::with_props(props).render();
}
