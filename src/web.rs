// NOTE: This file was generated with heavy LLM assistance.
//       Treat the code as experimental and review before trusting it.

#![cfg(feature = "web")]

use base16_palettes::{
    Palette,
    palettes::{DefaultDark, DefaultPalette},
};
use ratatui::Frame;
use webatui::prelude::*;
use yew::{Context, Renderer};

use crate::{
    config,
    game::{Action, Game, Input},
    render,
    theme::Theme,
};

impl TerminalApp for Game {
    type Message = Input;

    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_variables))]
    fn setup(&mut self, ctx: &Context<WebTerminal<Self>>) {
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::{
                KeyboardEvent,
                wasm_bindgen::{JsCast, prelude::Closure},
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
                let closure =
                    Closure::wrap(
                        Box::new(move |event: KeyboardEvent| match event.key().as_str() {
                            "q" | "Q" => callback.emit(Input::Quit),
                            "h" | "H" => callback.emit(Input::MoveLeft),
                            "l" | "L" => callback.emit(Input::MoveRight),
                            "k" | "K" => callback.emit(Input::MoveUp),
                            "j" | "J" => callback.emit(Input::MoveDown),
                            "y" | "Y" => callback.emit(Input::MoveUpLeft),
                            "u" | "U" => callback.emit(Input::MoveUpRight),
                            "b" | "B" => callback.emit(Input::MoveDownLeft),
                            "n" | "N" => callback.emit(Input::MoveDownRight),
                            ">" => callback.emit(Input::Descend),
                            _ => {}
                        }) as Box<dyn FnMut(_)>,
                    );

                let _ = window
                    .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());

                closure.forget();
            }
        }
    }

    fn update(&mut self, _ctx: TermContext<'_, Self>, input: Self::Message) -> bool {
        match self.handle_input(input) {
            Action::Redraw => true,
            Action::Quit => false,
            Action::Descend => {
                #[cfg(target_arch = "wasm32")]
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href(config::DESCEND_REDIRECT_URL);
                }
                false
            }
        }
    }

    fn render(&self, area: ratatui::layout::Rect, frame: &mut Frame<'_>) {
        let theme = Theme::default();
        render::draw(self, area, frame, &theme);
    }
}

pub fn run() {
    let palette = Palette::DefaultPalette(DefaultPalette::DefaultDark(DefaultDark));
    let props = WebTermProps::new_with_palette(Game::default(), palette);
    Renderer::<WebTerminal<Game>>::with_props(props).render();
}
