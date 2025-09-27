use base16_palettes::{
    palettes::{DefaultDark, DefaultPalette},
    Base16Accent, Base16Color, Base16Palette,
};
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::game::{Game, Tile};

pub fn draw(game: &Game, area: Rect, frame: &mut Frame<'_>) {
    let max_height = area.height as usize;
    let max_width = area.width as usize;

    let bg = palette_color(Base16Color::dark_1());
    let player_style = Style::default()
        .fg(palette_color(Base16Color::Accent(Base16Accent::Accent05)))
        .bg(bg);

    let mut lines = Vec::with_capacity(max_height);
    for y in 0..max_height {
        let mut spans = Vec::with_capacity(max_width);
        for x in 0..max_width {
            let span = if game.player.x as usize == x && game.player.y as usize == y {
                Span::styled("@", player_style)
            } else if y < game.map_height() && x < game.map_width() {
                render_tile(&game.tile_at(x, y), bg)
            } else {
                Span::styled(" ", Style::default().bg(bg))
            };
            spans.push(span);
        }
        lines.push(Line::from(spans));
    }

    let para = Paragraph::new(lines).style(Style::default().bg(bg));
    frame.render_widget(para, area);
}

fn render_tile(tile: &Tile, bg: Color) -> Span<'static> {
    match tile {
        Tile::Empty => Span::styled(" ", Style::default().bg(bg)),
        Tile::Wall => Span::styled(
            "#",
            Style::default()
                .fg(palette_color(Base16Color::Accent(Base16Accent::Accent01)))
                .bg(bg),
        ),
        Tile::Floor => Span::styled(
            ".",
            Style::default()
                .fg(palette_color(Base16Color::dark_3()))
                .bg(bg),
        ),
    }
}

fn palette_color(color: Base16Color) -> Color {
    const PALETTE: DefaultPalette = DefaultPalette::DefaultDark(DefaultDark);
    let (r, g, b) = PALETTE.to_rgb(color);
    Color::Rgb(r, g, b)
}
