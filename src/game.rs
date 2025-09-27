// NOTE: This file was generated with heavy LLM assistance.
//       Treat the code as experimental and review before trusting it.

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 25;

#[derive(Clone, PartialEq)]
pub struct Game {
    pub player: Position,
    map: Vec<Vec<Tile>>,
    monsters: Vec<Monster>,
    messages: Vec<String>,
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
    Descend,
}

pub enum Action {
    Quit,
    Redraw,
    Descend,
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
    StairsDown,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Monster {
    pub kind: MonsterKind,
    pub position: Position,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MonsterKind {
    Dog,
}

impl Default for Game {
    fn default() -> Self {
        let mut map = vec![vec![Tile::Empty; MAP_WIDTH]; MAP_HEIGHT];

        let room_width = 20;
        let room_height = 7;
        let room_x = (MAP_WIDTH.saturating_sub(room_width)) / 2;
        let room_y = (MAP_HEIGHT.saturating_sub(room_height)) / 2;
        carve_room(&mut map, room_x, room_y, room_width, room_height);

        let player = Position {
            x: (room_x + room_width / 2) as u16,
            y: (room_y + room_height / 2) as u16,
        };

        let stairs_x = (room_x + room_width - 3).min(room_x + room_width - 2);
        let stairs_y = room_y + room_height - 3;
        map[stairs_y][stairs_x] = Tile::StairsDown;

        let dog = Monster {
            kind: MonsterKind::Dog,
            position: Position {
                x: (room_x + 3) as u16,
                y: (room_y + 2) as u16,
            },
        };

        Self {
            player,
            map,
            monsters: vec![dog],
            messages: Vec::new(),
        }
    }
}

impl Game {
    pub fn handle_input(&mut self, input: Input) -> Action {
        self.messages.clear();

        let action = match input {
            Input::Quit => Action::Quit,
            Input::MoveLeft => self.try_move(-1, 0),
            Input::MoveRight => self.try_move(1, 0),
            Input::MoveUp => self.try_move(0, -1),
            Input::MoveDown => self.try_move(0, 1),
            Input::MoveUpLeft => self.try_move(-1, -1),
            Input::MoveUpRight => self.try_move(1, -1),
            Input::MoveDownLeft => self.try_move(-1, 1),
            Input::MoveDownRight => self.try_move(1, 1),
            Input::Descend => self.try_descend(),
        };

        if !matches!(action, Action::Quit | Action::Descend) {
            self.update_monsters();
        }

        action
    }

    fn try_move(&mut self, dx: i16, dy: i16) -> Action {
        let new_x = self.player.x as i16 + dx;
        let new_y = self.player.y as i16 + dy;

        if new_x < 0 || new_y < 0 {
            return Action::Redraw;
        }

        let (nx, ny) = (new_x as usize, new_y as usize);
        if nx >= MAP_WIDTH || ny >= MAP_HEIGHT {
            self.push_message("Ouch!");
            return Action::Redraw;
        }

        if self
            .monsters
            .iter()
            .any(|m| m.position.x as usize == nx && m.position.y as usize == ny)
        {
            return Action::Redraw;
        }

        if !matches!(self.map[ny][nx], Tile::Floor | Tile::StairsDown) {
            self.push_message("Ouch!");
            return Action::Redraw;
        }

        self.player.x = nx as u16;
        self.player.y = ny as u16;

        if matches!(self.map[ny][nx], Tile::StairsDown) {
            self.push_message("There seems to be a staircase here leading down");
        }

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

    fn current_tile(&self) -> Tile {
        self.map[self.player.y as usize][self.player.x as usize]
    }

    pub fn combined_message(&self) -> Option<String> {
        if self.messages.is_empty() {
            None
        } else {
            Some(self.messages.join("  "))
        }
    }

    pub fn monsters(&self) -> &[Monster] {
        &self.monsters
    }

    fn try_descend(&mut self) -> Action {
        if matches!(self.current_tile(), Tile::StairsDown) {
            self.push_message("Going down!");
            Action::Descend
        } else {
            self.push_message("No staircase here.");
            Action::Redraw
        }
    }

    fn update_monsters(&mut self) {
        for i in 0..self.monsters.len() {
            let (dx, dy) = self.monster_delta_towards_player(i);

            if dx == 0 && dy == 0 {
                self.push_message("The dog bites you!");
                continue;
            }

            let new_x = self.monsters[i].position.x as i16 + dx;
            let new_y = self.monsters[i].position.y as i16 + dy;

            if new_x < 0 || new_y < 0 {
                continue;
            }

            let new_pos = Position {
                x: new_x as u16,
                y: new_y as u16,
            };

            if new_pos == self.player {
                self.push_message("The dog bites you!");
                continue;
            }

            if self.is_blocked(new_pos) {
                continue;
            }

            if self
                .monsters
                .iter()
                .enumerate()
                .any(|(j, m)| j != i && m.position == new_pos)
            {
                continue;
            }

            self.monsters[i].position = new_pos;
        }
    }

    fn monster_delta_towards_player(&self, index: usize) -> (i16, i16) {
        let monster = self.monsters[index].position;
        let dx = (self.player.x as i16 - monster.x as i16).clamp(-1, 1);
        let dy = (self.player.y as i16 - monster.y as i16).clamp(-1, 1);
        (dx, dy)
    }

    fn is_blocked(&self, pos: Position) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;

        if x >= MAP_WIDTH || y >= MAP_HEIGHT {
            return true;
        }

        !matches!(self.map[y][x], Tile::Floor | Tile::StairsDown)
    }

    fn push_message(&mut self, msg: &str) {
        self.messages.push(msg.to_string());
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
