// NOTE: This file was generated with heavy LLM assistance.
//       Treat the code as experimental and review before trusting it.

use base16_palettes::{
    Base16Accent, Base16Color, Base16Palette,
    palettes::{DefaultDark, DefaultPalette},
};
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::game::{Game, Monster, MonsterKind, Tile};

pub fn draw(game: &Game, area: Rect, frame: &mut Frame<'_>) {
    let max_height = area.height as usize;
    let max_width = area.width as usize;

    let bg = palette_color(Base16Color::dark_1());
    let player_style = Style::default()
        .fg(palette_color(Base16Color::light_4()))
        .bg(bg);

    let mut lines = Vec::with_capacity(max_height);
    let message_text = game.combined_message();

    for y in 0..max_height {
        let mut spans = Vec::with_capacity(max_width);

        let map_y = y;
        for x in 0..max_width {
            let span = if game.player.x as usize == x && game.player.y as usize == map_y {
                Span::styled("@", player_style)
            } else if let Some(monster) = game
                .monsters()
                .iter()
                .find(|m| m.position.x as usize == x && m.position.y as usize == map_y)
            {
                render_monster(monster, bg)
            } else if map_y < game.map_height() && x < game.map_width() {
                render_tile(&game.tile_at(x, map_y), bg)
            } else {
                Span::styled(" ", Style::default().bg(bg))
            };
            spans.push(span);
        }

        lines.push(Line::from(spans));
    }

    if let Some(msg) = message_text {
        if let Some(line) = lines.get_mut(0) {
            let spans = &mut line.spans;
            let mut end = 0;
            for (idx, ch) in msg.chars().enumerate() {
                if idx >= spans.len() {
                    break;
                }
                spans[idx] = Span::styled(
                    ch.to_string(),
                    Style::default()
                        .fg(palette_color(Base16Color::Accent(Base16Accent::Accent04)))
                        .bg(bg),
                );
                end = idx + 1;
            }
            for idx in end..spans.len() {
                spans[idx] = Span::styled(" ", Style::default().bg(bg));
            }
        }
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
        Tile::StairsDown => Span::styled(
            ">",
            Style::default()
                .fg(palette_color(Base16Color::light_1()))
                .bg(bg),
        ),
    }
}

fn render_monster(monster: &Monster, bg: Color) -> Span<'static> {
    match monster.kind {
        MonsterKind::Dog => Span::styled(
            "d",
            Style::default()
                .fg(palette_color(Base16Color::Accent(Base16Accent::Accent00)))
                .bg(bg),
        ),
    }
}

fn palette_color(color: Base16Color) -> Color {
    const PALETTE: DefaultPalette = DefaultPalette::DefaultDark(DefaultDark);
    let (r, g, b) = PALETTE.to_rgb(color);
    Color::Rgb(r, g, b)
}
