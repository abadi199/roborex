use constant::{PLAYER_Z, SCALING_FACTOR, WALKING_DURATION};
use direction::Direction;
use game_map::GameMap;
use grid::Grid;
use player_state::PlayerState;
use primitive::Position;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Image},
    input::{ButtonState, Key, Keyboard, MouseButton},
    lifecycle::{Asset, Window},
    Result,
};

pub struct Player {
    pub position: Position,
    state: PlayerState,
    framerate: u32,
    standing_side_sprites: Vec<Asset<Image>>,
    standing_up_sprites: Vec<Asset<Image>>,
    standing_down_sprites: Vec<Asset<Image>>,
    standing_sprites_idx: usize,
    standing_tick: f64,
    walking_side_sprites: Vec<Asset<Image>>,
    walking_up_sprites: Vec<Asset<Image>>,
    walking_down_sprites: Vec<Asset<Image>>,
}

impl Player {
    pub fn new() -> Self {
        let standing_side_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/still-side1.png")),
            Asset::new(Image::load("resources/images/still-side2.png")),
        ];
        let standing_up_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/still-up1.png")),
            Asset::new(Image::load("resources/images/still-up2.png")),
        ];
        let standing_down_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/still-down1.png")),
            Asset::new(Image::load("resources/images/still-down2.png")),
        ];

        let walking_side_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/walking-side1.png")),
            Asset::new(Image::load("resources/images/walking-side2.png")),
            Asset::new(Image::load("resources/images/walking-side3.png")),
            Asset::new(Image::load("resources/images/walking-side4.png")),
        ];
        let walking_up_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/walking-up1.png")),
            Asset::new(Image::load("resources/images/walking-up2.png")),
            Asset::new(Image::load("resources/images/walking-up3.png")),
            Asset::new(Image::load("resources/images/walking-up4.png")),
        ];
        let walking_down_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/walking-down1.png")),
            Asset::new(Image::load("resources/images/walking-down2.png")),
            Asset::new(Image::load("resources/images/walking-down3.png")),
            Asset::new(Image::load("resources/images/walking-down4.png")),
        ];

        Player {
            position: Position::new(0, 0),
            state: PlayerState::Standing(Direction::Right),
            framerate: 5,
            standing_side_sprites,
            standing_up_sprites,
            standing_down_sprites,
            standing_sprites_idx: 0,
            standing_tick: 0.,
            walking_side_sprites,
            walking_up_sprites,
            walking_down_sprites,
        }
    }

    fn is_walking_button_pressed(keyboard: &Keyboard) -> bool {
        keyboard[Key::Left].is_down()
            || keyboard[Key::Right].is_down()
            || keyboard[Key::Up].is_down()
            || keyboard[Key::Down].is_down()
    }

    pub fn update(&mut self, window: &mut Window, game_map: &GameMap) -> Result<()> {
        let update_rate = window.update_rate();
        self.standing_tick += update_rate;

        if let PlayerState::Walking {
            direction,
            sprites_idx,
            tick,
            timer,
            grid_count,
        } = self.state
        {
            if !game_map.can_walk_to(&self.next_position(direction)) {
                self.stop();
                return Ok(());
            }
            if timer <= 0. {
                match direction {
                    Direction::Right => self.position.x += 1,
                    Direction::Left if self.position.x < 1 => {}
                    Direction::Left => self.position.x -= 1,
                    Direction::Up if self.position.y < 1 => {}
                    Direction::Up => self.position.y -= 1,
                    Direction::Down => self.position.y += 1,
                };
                if grid_count > 1 {
                    self.state.grid_count(grid_count - 1);
                    self.state.timer(WALKING_DURATION);
                } else if !Self::is_walking_button_pressed(window.keyboard()) {
                    self.stop();
                } else {
                    if window.keyboard()[Key::Right].is_down() {
                        self.walk(Direction::Right, game_map);
                    }

                    if window.keyboard()[Key::Left].is_down() {
                        self.walk(Direction::Left, game_map);
                    }

                    if window.keyboard()[Key::Up].is_down() {
                        self.walk(Direction::Up, game_map);
                    }

                    if window.keyboard()[Key::Down].is_down() {
                        self.walk(Direction::Down, game_map);
                    }
                }
            } else {
                let updated_tick = tick + update_rate;
                self.state.tick(updated_tick);
                self.state.timer(timer - update_rate);

                if updated_tick > (1000. / (self.framerate as f64)) {
                    self.state.sprites_idx(sprites_idx + 1);
                    self.state.tick(0.);
                }
            }
        }

        if self.standing_tick > (1000. / (self.framerate as f64)) {
            self.standing_sprites_idx += 1;
            self.standing_tick = 0.;
        }

        if window.keyboard()[Key::Right].is_down() && !self.is_walking() {
            self.walk(Direction::Right, game_map);
        }

        if window.keyboard()[Key::Left].is_down() && !self.is_walking() {
            self.walk(Direction::Left, game_map);
        }

        if window.keyboard()[Key::Up].is_down() && !self.is_walking() {
            self.walk(Direction::Up, game_map);
        }

        if window.keyboard()[Key::Down].is_down() && !self.is_walking() {
            self.walk(Direction::Down, game_map);
        }

        if window.mouse()[MouseButton::Left] == ButtonState::Released && !self.is_walking() {
            let mouse_pos = window.mouse().pos();
            let grid_coordinate = Grid::from_coordinate(mouse_pos);
            self.walk_to(grid_coordinate);
        }

        Ok(())
    }

    fn is_walking(&self) -> bool {
        match self.state {
            PlayerState::Walking { .. } => true,
            PlayerState::Standing(_) => false,
        }
    }

    fn next_position(&self, direction: Direction) -> Position {
        match direction {
            Direction::Right => self.position.add(1, 0),
            Direction::Left if self.position.x < 1 => self.position.clone(),
            Direction::Left => self.position.add(-1, 0),
            Direction::Up if self.position.y < 1 => self.position.clone(),
            Direction::Up => self.position.add(0, -1),
            Direction::Down => self.position.add(0, 1),
        }
    }

    fn stop(&mut self) {
        self.state = PlayerState::stop(&self.state);
    }

    fn walk(&mut self, direction: Direction, game_map: &GameMap) {
        let next_position = self.next_position(direction);
        if game_map.can_walk_to(&next_position) && self.position != next_position {
            self.standing_tick = 0.;
            self.standing_sprites_idx = 0;
            self.state = PlayerState::Walking {
                direction,
                grid_count: 1,
                timer: WALKING_DURATION,
                sprites_idx: 0,
                tick: 0.,
            };
        } else {
            self.stop();
        }
    }

    fn walk_to(&mut self, next_position: Position) {
        let (direction_x, grid_count_x) = match next_position.x > self.position.x {
            true => (Direction::Right, next_position.x - self.position.x),
            false if next_position.x < self.position.x => {
                (Direction::Left, self.position.x - next_position.x)
            }
            _ => (Direction::Right, 0),
        };

        let (direction_y, grid_count_y) = match next_position.y > self.position.y {
            true => (Direction::Down, next_position.y - self.position.y),
            false if next_position.y < self.position.y => {
                (Direction::Up, self.position.y - next_position.y)
            }
            _ => (Direction::Right, 0),
        };

        self.state = match (grid_count_x, grid_count_y) {
            (0, 0) => PlayerState::stop(&self.state),
            _ if grid_count_x >= grid_count_y => PlayerState::Walking {
                direction: direction_x,
                grid_count: grid_count_x,
                timer: WALKING_DURATION,
                sprites_idx: 0,
                tick: 0.,
            },
            _ => PlayerState::Walking {
                direction: direction_y,
                grid_count: grid_count_y,
                timer: WALKING_DURATION,
                sprites_idx: 0,
                tick: 0.,
            },
        };
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let player_coordinate = Grid::to_player_coordinate(&self.state, &self.position);
        let scale = Transform::scale(Vector::new(SCALING_FACTOR, SCALING_FACTOR));
        let flip = Transform::scale(Vector::new(-1, 1));
        let transformation = match self.state {
            PlayerState::Standing(Direction::Up)
            | PlayerState::Standing(Direction::Down)
            | PlayerState::Standing(Direction::Right)
            | PlayerState::Walking {
                direction: Direction::Up,
                ..
            }
            | PlayerState::Walking {
                direction: Direction::Down,
                ..
            }
            | PlayerState::Walking {
                direction: Direction::Right,
                ..
            } => scale,
            PlayerState::Standing(Direction::Left)
            | PlayerState::Walking {
                direction: Direction::Left,
                ..
            } => scale * flip,
        };

        let image = match self.state {
            PlayerState::Standing(Direction::Left) | PlayerState::Standing(Direction::Right) => {
                let standing_sprites_idx =
                    self.standing_sprites_idx % self.standing_side_sprites.len();
                &mut self.standing_side_sprites[standing_sprites_idx]
            }
            PlayerState::Standing(Direction::Up) => {
                let standing_sprites_idx =
                    self.standing_sprites_idx % self.standing_up_sprites.len();
                &mut self.standing_up_sprites[standing_sprites_idx]
            }
            PlayerState::Standing(Direction::Down) => {
                let standing_sprites_idx =
                    self.standing_sprites_idx % self.standing_down_sprites.len();
                &mut self.standing_down_sprites[standing_sprites_idx]
            }
            PlayerState::Walking {
                direction: Direction::Left,
                sprites_idx,
                ..
            }
            | PlayerState::Walking {
                direction: Direction::Right,
                sprites_idx,
                ..
            } => {
                let walking_sprites_idx = sprites_idx % self.walking_side_sprites.len();
                &mut self.walking_side_sprites[walking_sprites_idx]
            }
            PlayerState::Walking {
                direction: Direction::Down,
                sprites_idx,
                ..
            } => {
                let walking_sprites_idx = sprites_idx % self.walking_down_sprites.len();
                &mut self.walking_down_sprites[walking_sprites_idx]
            }
            PlayerState::Walking {
                direction: Direction::Up,
                sprites_idx,
                ..
            } => {
                let walking_sprites_idx = sprites_idx % self.walking_up_sprites.len();
                &mut self.walking_up_sprites[walking_sprites_idx]
            }
        };
        image.execute(|image| {
            window.draw_ex(
                &image.area().with_center(player_coordinate),
                Img(&image),
                transformation,
                PLAYER_Z,
            );
            Ok(())
        })
    }
}
