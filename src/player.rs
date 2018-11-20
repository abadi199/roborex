use constant::WALKING_DURATION;
use direction::Direction;
use grid::Grid;
use player_state::PlayerState;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Image},
    input::{Key, Keyboard},
    lifecycle::{Asset, Window},
    Result,
};

pub struct Player {
    position: (u32, u32),
    state: PlayerState,
    framerate: u32,
    standing_sprites: Vec<Asset<Image>>,
    standing_sprites_idx: usize,
    standing_tick: f64,
    walking_sprites: Vec<Asset<Image>>,
}

impl Player {
    pub fn new() -> Self {
        let standing_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/DinoStill1.png")),
            Asset::new(Image::load("resources/images/DinoStill2.png")),
            Asset::new(Image::load("resources/images/DinoStill3.png")),
        ];

        let walking_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/DinoWalk1.png")),
            Asset::new(Image::load("resources/images/DinoWalk2.png")),
            Asset::new(Image::load("resources/images/DinoWalk3.png")),
        ];

        Player {
            position: (0, 0),
            state: PlayerState::Standing(Direction::Right),
            framerate: 10,
            standing_sprites,
            standing_sprites_idx: 0,
            standing_tick: 0.,
            walking_sprites,
        }
    }

    fn is_walking_button_pressed(keyboard: &Keyboard) -> bool {
        keyboard[Key::Left].is_down()
            || keyboard[Key::Right].is_down()
            || keyboard[Key::Up].is_down()
            || keyboard[Key::Down].is_down()
    }

    pub fn update(&mut self, window: &mut Window) -> Result<()> {
        let update_rate = window.update_rate();
        self.standing_tick += update_rate;

        if let PlayerState::Walking {
            direction,
            sprites_idx,
            tick,
            timer,
            ..
        } = self.state
        {
            if timer <= 0. {
                if !Self::is_walking_button_pressed(window.keyboard()) {
                    self.state = self.state.stop();
                    match direction {
                        Direction::Right => self.position.0 += 1,
                        Direction::Left if self.position.0 < 1 => {}
                        Direction::Left => self.position.0 -= 1,
                        Direction::Up if self.position.1 < 1 => {}
                        Direction::Up => self.position.1 -= 1,
                        Direction::Down => self.position.1 += 1,
                    };
                } else {
                    if window.keyboard()[Key::Right].is_down() && !self.is_walking() {
                        self.walk(Direction::Right);
                    }

                    if window.keyboard()[Key::Left].is_down() && !self.is_walking() {
                        self.walk(Direction::Left);
                    }

                    if window.keyboard()[Key::Up].is_down() && !self.is_walking() {
                        self.walk(Direction::Up);
                    }

                    if window.keyboard()[Key::Down].is_down() && !self.is_walking() {
                        self.walk(Direction::Down);
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
            self.walk(Direction::Right);
        }

        if window.keyboard()[Key::Left].is_down() && !self.is_walking() {
            self.walk(Direction::Left);
        }

        if window.keyboard()[Key::Up].is_down() && !self.is_walking() {
            self.walk(Direction::Up);
        }

        if window.keyboard()[Key::Down].is_down() && !self.is_walking() {
            self.walk(Direction::Down);
        }

        // if !window.keyboard()[key::up].is_down()
        //     && !window.keyboard()[key::down].is_down()
        //     && !window.keyboard()[key::left].is_down()
        //     && !window.keyboard()[key::right].is_down()
        // {
        //     self.state = self.state.stop();
        // }

        Ok(())
    }

    fn is_walking(&self) -> bool {
        match self.state {
            PlayerState::Walking { .. } => true,
            PlayerState::Standing(_) => false,
        }
    }

    fn walk(&mut self, direction: Direction) {
        self.standing_tick = 0.;
        self.standing_sprites_idx = 0;
        self.state = PlayerState::Walking {
            direction,
            grid_count: 1,
            timer: WALKING_DURATION,
            sprites_idx: 0,
            tick: 0.,
        };
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let player_coordinate = Grid::to_player_coordinate(&self.state, self.position);
        let scale = Transform::scale(Vector::new(0.2, 0.2));
        let flip = Transform::scale(Vector::new(-1, 1)) * Transform::translate(Vector::new(64, 0));
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
            PlayerState::Standing(_) => {
                let standing_sprites_idx = self.standing_sprites_idx % self.standing_sprites.len();
                &mut self.standing_sprites[standing_sprites_idx]
            }
            PlayerState::Walking { sprites_idx, .. } => {
                let walking_sprites_idx = sprites_idx % self.walking_sprites.len();
                &mut self.walking_sprites[walking_sprites_idx]
            }
        };
        image.execute(|image| {
            window.draw_ex(
                &image.area().with_center(player_coordinate),
                Img(&image),
                transformation,
                1,
            );
            Ok(())
        })
    }
}
