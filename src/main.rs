// Draw some multi-colored geometry to the screen
extern crate nalgebra;
extern crate quicksilver;
extern crate tiled;

use quicksilver::{
    geom::{Rectangle, Shape, Transform, Vector},
    graphics::{Background::Img, Color, Image},
    input::{ButtonState, Key},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::parse;

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const WALKING_DURATION: f64 = 500.;

// A unit struct that we're going to use to run the Quicksilver functions
struct RoboRex {
    time: f64,
    player: Player,
    game_map: GameMap,
}

#[derive(Debug)]
enum PlayerState {
    Standing(Direction),
    Walking {
        direction: Direction,
        grid_count: u32,
        timer: f64,
        sprites_idx: usize,
        tick: f64,
    },
}

impl PlayerState {
    fn sprites_idx(&mut self, new_sprites_idx: usize) {
        if let PlayerState::Walking {
            ref mut sprites_idx,
            ..
        } = self
        {
            *sprites_idx = new_sprites_idx;
        }
    }

    fn timer(&mut self, new_timer: f64) {
        if let PlayerState::Walking { ref mut timer, .. } = self {
            *timer = new_timer;
        }
    }

    fn grid_count(&mut self, new_grid_count: u32) {
        if let PlayerState::Walking {
            ref mut grid_count, ..
        } = self
        {
            *grid_count = new_grid_count;
        }
    }

    fn tick(&mut self, new_tick: f64) {
        if let PlayerState::Walking { ref mut tick, .. } = self {
            *tick = new_tick;
        }
    }
}

impl PlayerState {
    fn stop(&self) -> Self {
        match self {
            PlayerState::Walking { direction, .. } => PlayerState::Standing(direction.clone()),
            PlayerState::Standing(direction) => PlayerState::Standing(direction.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
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

    fn up(speed: i32) -> Self {
        Motion {
            velocity: Transform::scale(Vector::new(speed, speed)) * Vector::new(0, -1),
            acceleration: Vector::new(0, 0),
        }
    }

    fn down(speed: i32) -> Self {
        Motion {
            velocity: Transform::scale(Vector::new(speed, speed)) * Vector::new(0, 1),
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

struct GameLayer {
    rectangles: Vec<Vec<Option<Rectangle>>>,
    image: Asset<Image>,
}

impl GameLayer {
    fn draw(&mut self, window: &mut Window) {
        let rectangles = &self.rectangles;
        self.image.execute(|image| {
            for (y, row) in rectangles.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    if let Some(rec) = col {
                        let draw_rec = Grid::to_rectangle(x as u32, y as u32);
                        window.draw_ex(
                            &draw_rec,
                            Img(&image.subimage(*rec)),
                            Transform::scale(Vector::new(2, 2)),
                            1,
                        );
                    }
                }
            }
            Ok(())
        });
    }
}

enum Grid {
    Path,
    NonPath,
}

impl Grid {
    fn to_rectangle(x: u32, y: u32) -> Rectangle {
        Rectangle::new(
            ((x * TILE_WIDTH * 2) + 8, (y * TILE_HEIGHT * 2) + 8),
            (TILE_WIDTH, TILE_HEIGHT),
        )
    }

    fn to_coordinate((x, y): (u32, u32)) -> Vector {
        Vector::new((x * TILE_WIDTH * 2) + 8, (y * TILE_HEIGHT * 2) + 8)
    }

    fn to_player_coordinate(state: &PlayerState, (x, y): (u32, u32)) -> Vector {
        let (delta_x, delta_y) = match state {
            PlayerState::Walking {
                direction: Direction::Right,
                timer,
                ..
            } => (((WALKING_DURATION - timer) / WALKING_DURATION) * 32., 0.),
            PlayerState::Walking {
                direction: Direction::Left,
                timer,
                ..
            } => (((WALKING_DURATION - timer) / WALKING_DURATION) * -32., 0.),
            PlayerState::Walking {
                direction: Direction::Up,
                timer,
                ..
            } => (0., ((WALKING_DURATION - timer) / WALKING_DURATION) * -32.),
            PlayerState::Walking {
                direction: Direction::Down,
                timer,
                ..
            } => (0., ((WALKING_DURATION - timer) / WALKING_DURATION) * 32.),
            _ => (0., 0.),
        };

        Vector::new(
            (((x * TILE_WIDTH * 2) + 40) as i32 + delta_x as i32) as u32,
            (((y * TILE_HEIGHT * 2) + 24) as i32 + delta_y as i32) as u32,
        )
    }
}

struct GameMap {
    layers: Vec<GameLayer>,
    grid: Vec<Vec<Grid>>,
}

impl GameMap {
    fn load(map: tiled::Map) -> Self {
        let layers: Vec<GameLayer> = map
            .layers
            .iter()
            .map(|layer| {
                Self::to_game_layer(
                    &layer,
                    map.tile_width,
                    map.tile_height,
                    &map.tilesets[0].images[0],
                )
            })
            .collect();
        GameMap {
            layers,
            grid: vec![],
        }
    }

    fn draw(&mut self, window: &mut Window) {
        self.layers[0].draw(window);
        self.layers[1].draw(window);
        self.layers[2].draw(window);
    }

    fn to_game_layer(
        layer: &tiled::Layer,
        tile_width: u32,
        tile_height: u32,
        image: &tiled::Image,
    ) -> GameLayer {
        let rectangles = layer
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile: &u32| Self::to_rectangle(*tile, tile_width, tile_height, image))
                    .collect()
            })
            .collect();
        let image = Asset::new(Image::load(format!("resources/tiled/{}", image.source)));
        GameLayer { rectangles, image }
    }

    fn to_rectangle(
        tile: u32,
        tile_width: u32,
        tile_height: u32,
        image: &tiled::Image,
    ) -> Option<Rectangle> {
        match tile {
            0 => None,
            _ => Some(Rectangle::new(
                (
                    ((tile % (image.width as u32 / tile_width) * tile_width) - tile_width) as f32,
                    (tile / (image.width as u32 / tile_height) * tile_height) as f32,
                ),
                (tile_width as f32, tile_height as f32),
            )),
        }
    }
}

struct Player {
    position: (u32, u32),
    state: PlayerState,
    framerate: u32,
    speed: i32,
    standing_sprites: Vec<Asset<Image>>,
    standing_sprites_idx: usize,
    standing_tick: f64,
    walking_sprites: Vec<Asset<Image>>,
}

impl Player {
    fn new() -> Self {
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
            speed: 3,
            standing_sprites,
            standing_sprites_idx: 0,
            standing_tick: 0.,
            walking_sprites,
        }
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
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

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let mut player_coordinate = Grid::to_player_coordinate(&self.state, self.position);
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

impl State for RoboRex {
    // Initialize the struct
    fn new() -> Result<RoboRex> {
        let file = File::open(&Path::new("resources/tiled/level.tmx")).unwrap();
        println!("Opened file");
        let reader = BufReader::new(file);
        let map = parse(reader).unwrap();
        let game_map = GameMap::load(map);

        Ok(RoboRex {
            time: 0.,
            player: Player::new(),
            game_map,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.time += window.update_rate();
        self.player.update(window)?;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.game_map.draw(window);
        self.player.draw(window)?;
        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    // Run with DrawGeometry as the event handler, with a window title of 'Draw Geometry' and a
    // size of (800, 600)
    run::<RoboRex>("RoboRex", Vector::new(512, 512), Settings::default());
}
