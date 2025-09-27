// NOTE: This file was generated with heavy LLM assistance.
//       Treat the code as experimental and review before trusting it.

use base16_palettes::{Base16Color, Base16Palette, Palette};
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    Frame,
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    game::{Game, Monster, MonsterKind, Tile},
    theme::Theme,
};

pub fn draw(game: &Game, area: Rect, frame: &mut Frame<'_>, theme: &Theme) {
    let max_height = area.height as usize;
    let max_width = area.width as usize;

    let palette = theme.palette();
    let bg = palette_color(theme.background, &palette);
    let player_style = Style::default()
        .fg(palette_color(theme.player, &palette))
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
                render_monster(monster, bg, theme, &palette)
            } else if map_y < game.map_height() && x < game.map_width() {
                render_tile(&game.tile_at(x, map_y), bg, theme, &palette)
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
                        .fg(palette_color(theme.message, &palette))
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

fn render_tile(tile: &Tile, bg: Color, theme: &Theme, palette: &Palette) -> Span<'static> {
    match tile {
        Tile::Empty => Span::styled(" ", Style::default().bg(bg)),
        Tile::Wall => Span::styled(
            "#",
            Style::default()
                .fg(palette_color(theme.wall, palette))
                .bg(bg),
        ),
        Tile::Floor => Span::styled(
            ".",
            Style::default()
                .fg(palette_color(theme.floor, palette))
                .bg(bg),
        ),
        Tile::StairsDown => Span::styled(
            ">",
            Style::default()
                .fg(palette_color(theme.stairs_down, palette))
                .bg(bg),
        ),
    }
}

fn render_monster(monster: &Monster, bg: Color, theme: &Theme, palette: &Palette) -> Span<'static> {
    let color = theme.monster(monster.kind);
    let glyph = match monster.kind {
        MonsterKind::Dog => "d",
    };

    Span::styled(
        glyph,
        Style::default().fg(palette_color(color, palette)).bg(bg),
    )
}

fn palette_color(color: Base16Color, palette: &Palette) -> Color {
    let (r, g, b) = palette.to_rgb(color);
    Color::Rgb(r, g, b)
}
