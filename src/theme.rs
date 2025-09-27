// NOTE: This file was generated with heavy LLM assistance.
//       Treat the code as experimental and review before trusting it.

use base16_palettes::{
    Base16Color, Palette,
    palettes::{DefaultDark, DefaultPalette},
};

use crate::game::MonsterKind;

#[derive(Clone, Copy)]
pub struct Theme {
    pub background: Base16Color,
    pub player: Base16Color,
    pub stairs_down: Base16Color,
    pub wall: Base16Color,
    pub floor: Base16Color,
    pub message: Base16Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Base16Color::dark_1(),
            player: Base16Color::light_4(),
            stairs_down: Base16Color::light_1(),
            wall: Base16Color::Accent(base16_palettes::Base16Accent::Accent01),
            floor: Base16Color::dark_3(),
            message: Base16Color::Accent(base16_palettes::Base16Accent::Accent04),
        }
    }
}

impl Theme {
    pub fn monster(&self, kind: MonsterKind) -> Base16Color {
        match kind {
            MonsterKind::Dog => Base16Color::Accent(base16_palettes::Base16Accent::Accent00),
        }
    }

    pub fn palette(&self) -> Palette {
        Palette::DefaultPalette(DefaultPalette::DefaultDark(DefaultDark))
    }
}
