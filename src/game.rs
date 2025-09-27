const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 25;

#[derive(Clone, PartialEq)]
pub struct Game {
    pub player: Position,
    map: Vec<Vec<Tile>>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Quit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveUpLeft,
    MoveUpRight,
    MoveDownLeft,
    MoveDownRight,
}

pub enum Action {
    Quit,
    Redraw,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Floor,
}

impl Default for Game {
    fn default() -> Self {
        let mut map = vec![vec![Tile::Empty; MAP_WIDTH]; MAP_HEIGHT];

        let room_width = 20;
        let room_height = 10;
        let room_x = (MAP_WIDTH.saturating_sub(room_width)) / 2;
        let room_y = (MAP_HEIGHT.saturating_sub(room_height)) / 2;
        carve_room(&mut map, room_x, room_y, room_width, room_height);

        let player = Position {
            x: (room_x + room_width / 2) as u16,
            y: (room_y + room_height / 2) as u16,
        };

        Self { player, map }
    }
}

impl Game {
    pub fn handle_input(&mut self, input: Input) -> Action {
        match input {
            Input::Quit => Action::Quit,
            Input::MoveLeft => self.try_move(-1, 0),
            Input::MoveRight => self.try_move(1, 0),
            Input::MoveUp => self.try_move(0, -1),
            Input::MoveDown => self.try_move(0, 1),
            Input::MoveUpLeft => self.try_move(-1, -1),
            Input::MoveUpRight => self.try_move(1, -1),
            Input::MoveDownLeft => self.try_move(-1, 1),
            Input::MoveDownRight => self.try_move(1, 1),
        }
    }

    fn try_move(&mut self, dx: i16, dy: i16) -> Action {
        let new_x = self.player.x as i16 + dx;
        let new_y = self.player.y as i16 + dy;

        if new_x < 0 || new_y < 0 {
            return Action::Redraw;
        }

        let (nx, ny) = (new_x as usize, new_y as usize);
        if nx >= MAP_WIDTH || ny >= MAP_HEIGHT {
            return Action::Redraw;
        }

        if !matches!(self.map[ny][nx], Tile::Floor) {
            return Action::Redraw;
        }

        self.player.x = nx as u16;
        self.player.y = ny as u16;
        Action::Redraw
    }

    pub fn map_width(&self) -> usize {
        MAP_WIDTH
    }

    pub fn map_height(&self) -> usize {
        MAP_HEIGHT
    }

    pub fn tile_at(&self, x: usize, y: usize) -> Tile {
        self.map[y][x]
    }
}

fn carve_room(
    map: &mut [Vec<Tile>],
    origin_x: usize,
    origin_y: usize,
    width: usize,
    height: usize,
) {
    if width == 0 || height == 0 {
        return;
    }

    for y in 0..height {
        let map_y = origin_y + y;
        if map_y >= map.len() {
            break;
        }

        for x in 0..width {
            let map_x = origin_x + x;
            if map_x >= map[map_y].len() {
                break;
            }

            let is_border = y == 0 || x == 0 || y == height - 1 || x == width - 1;
            map[map_y][map_x] = if is_border { Tile::Wall } else { Tile::Floor };
        }
    }
}
