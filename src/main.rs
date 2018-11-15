// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    geom::{Circle, Line, Rectangle, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Background::Img, Color, Image},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

// A unit struct that we're going to use to run the Quicksilver functions
struct RoboRex {
    standing_sprite: Asset<Image>,
    walking_sprites: Vec<Asset<Image>>,
    walking_sprites_idx: usize,
    time: f64,
    player: Player,
}

struct Player {
    state: PlayerState,
}

enum PlayerState {
    Standing,
    Walking(Direction),
}

enum Direction {
    Left,
    Right,
}

impl Player {
    fn new() -> Self {
        Player {
            state: PlayerState::Standing,
        }
    }
}

impl State for RoboRex {
    // Initialize the struct
    fn new() -> Result<RoboRex> {
        let standing_sprite: Asset<Image> =
            Asset::new(Image::load("resources/images/DinoStill.png"));

        let walking_sprites: Vec<Asset<Image>> = vec![
            Asset::new(Image::load("resources/images/DinoWalk1.png")),
            Asset::new(Image::load("resources/images/DinoWalk2.png")),
            Asset::new(Image::load("resources/images/DinoWalk3.png")),
        ];

        Ok(RoboRex {
            standing_sprite,
            walking_sprites,
            walking_sprites_idx: 0,
            time: 0.,
            player: Player::new(),
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        let update_rate = window.update_rate();
        self.time += update_rate;
        self.walking_sprites_idx += 1;

        if window.keyboard()[Key::Right].is_down() {
            self.player.state = PlayerState::Walking(Direction::Right);
        } else if window.keyboard()[Key::Left].is_down() {
            self.player.state = PlayerState::Walking(Direction::Left);
        } else {
            self.walking_sprites_idx = 0;
            self.player.state = PlayerState::Standing;
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let walking_sprites_idx = (self.walking_sprites_idx / 10) % self.walking_sprites.len();
        match self.player.state {
            PlayerState::Standing => self.standing_sprite.execute(|image| {
                window.draw_ex(
                    &image.area().with_center((400, 300)),
                    Img(&image),
                    Transform::scale(Vector::new(0.5, 0.5)),
                    1,
                );
                Ok(())
            }),

            PlayerState::Walking(Direction::Right) => self.walking_sprites[walking_sprites_idx]
                .execute(|image| {
                    window.draw_ex(
                        &image.area().with_center((400, 300)),
                        Img(&image),
                        Transform::scale(Vector::new(0.5, 0.5)),
                        1,
                    );
                    Ok(())
                }),
            PlayerState::Walking(Direction::Left) => self.walking_sprites[walking_sprites_idx]
                .execute(|image| {
                    window.draw_ex(
                        &image.area().with_center((300, 300)),
                        Img(&image),
                        Transform::scale(Vector::new(-0.5, 0.5)),
                        1,
                    );
                    Ok(())
                }),
        };

        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    // Run with DrawGeometry as the event handler, with a window title of 'Draw Geometry' and a
    // size of (800, 600)
    run::<RoboRex>("RoboRex", Vector::new(800, 600), Settings::default());
}
