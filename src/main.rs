// Draw some multi-colored geometry to the screen
extern crate nalgebra;
extern crate quicksilver;

use nalgebra::geometry::Point2;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

// A unit struct that we're going to use to run the Quicksilver functions
struct RoboRex {
    standing_sprites: Vec<Asset<Image>>,
    standing_sprites_idx: usize,
    standing_tick: f64,
    walking_sprites: Vec<Asset<Image>>,
    walking_sprites_idx: usize,
    walking_tick: f64,
    time: f64,
    player: Player,
}

enum PlayerState {
    Standing(Direction),
    Walking(Direction),
}

impl PlayerState {
    fn stop(&self) -> Self {
        match self {
            PlayerState::Walking(direction) => PlayerState::Standing(direction.clone()),
            PlayerState::Standing(direction) => PlayerState::Standing(direction.clone()),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Motion {
    pub velocity: Vector,
    pub acceleration: Vector,
}

impl Motion {
    fn left(speed: i32) -> Self {
        Motion {
            velocity: Transform::scale(Vector::new(speed, speed)) * Vector::new(-1, 0),
            acceleration: Vector::new(0, 0),
        }
    }

    fn right(speed: i32) -> Self {
        Motion {
            velocity: Transform::scale(Vector::new(speed, speed)) * Vector::new(1, 0),
            acceleration: Vector::new(0, 0),
        }
    }

    fn new() -> Self {
        Motion {
            velocity: Vector::new(0, 0),
            acceleration: Vector::new(0, 0),
        }
    }

    fn is_left(&self) -> bool {
        self.velocity.x < 0. && self.velocity.y == 0.
    }

    fn is_right(&self) -> bool {
        self.velocity.x > 0. && self.velocity.y == 0.
    }
}

struct Player {
    position: Vector,
    state: PlayerState,
    framerate: u32,
    speed: i32,
}

impl Player {
    fn new() -> Self {
        Player {
            position: Vector::new(300, 300),
            state: PlayerState::Standing(Direction::Right),
            framerate: 10,
            speed: 3,
        }
    }
}

impl State for RoboRex {
    // Initialize the struct
    fn new() -> Result<RoboRex> {
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

        Ok(RoboRex {
            standing_sprites,
            standing_sprites_idx: 0,
            standing_tick: 0.,
            walking_sprites,
            walking_sprites_idx: 0,
            walking_tick: 0.,
            time: 0.,
            player: Player::new(),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let update_rate = window.update_rate();
        self.time += update_rate;
        self.walking_tick += update_rate;
        self.standing_tick += update_rate;
        if self.walking_tick > (1000. / (self.player.framerate as f64)) {
            self.walking_sprites_idx += 1;
            self.walking_tick = 0.;
        }

        if self.standing_tick > (1000. / (self.player.framerate as f64)) {
            self.standing_sprites_idx += 1;
            self.standing_tick = 0.;
        }

        if window.keyboard()[Key::Right].is_down() {
            self.standing_tick = 0.;
            self.standing_sprites_idx = 0;
            let motion = Motion::right(self.player.speed);
            self.player.state = PlayerState::Walking(Direction::Right);
            self.player.position = self.player.position + motion.velocity;
        } else if window.keyboard()[Key::Left].is_down() {
            self.standing_tick = 0.;
            self.standing_sprites_idx = 0;
            let motion = Motion::left(self.player.speed);
            self.player.state = PlayerState::Walking(Direction::Left);
            self.player.position = self.player.position + motion.velocity;
        } else {
            self.walking_tick = 0.;
            self.walking_sprites_idx = 0;
            self.player.state = self.player.state.stop();
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let player_position = self.player.position;
        let scale = Transform::scale(Vector::new(0.5, 0.5));
        let flip = Transform::scale(Vector::new(-1, 1)) * Transform::translate(Vector::new(64, 0));
        let transformation = match self.player.state {
            PlayerState::Standing(Direction::Right) => scale,
            PlayerState::Standing(Direction::Left) => scale * flip,
            PlayerState::Walking(Direction::Right) => scale,
            PlayerState::Walking(Direction::Left) => scale * flip,
        };

        let image = match self.player.state {
            PlayerState::Standing(_) => {
                let standing_sprites_idx = self.standing_sprites_idx % self.standing_sprites.len();
                &mut self.standing_sprites[standing_sprites_idx]
            }
            PlayerState::Walking(_) => {
                let walking_sprites_idx = self.walking_sprites_idx % self.walking_sprites.len();
                &mut self.walking_sprites[walking_sprites_idx]
            }
        };

        image.execute(|image| {
            window.draw_ex(
                &image.area().with_center(player_position),
                Img(&image),
                transformation,
                1,
            );
            Ok(())
        })
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    // Run with DrawGeometry as the event handler, with a window title of 'Draw Geometry' and a
    // size of (800, 600)
    run::<RoboRex>("RoboRex", Vector::new(800, 600), Settings::default());
}
