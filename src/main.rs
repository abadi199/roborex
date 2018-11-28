#[macro_use]
extern crate lazy_static;

extern crate futures;
extern crate nalgebra;
extern crate quicksilver;

// #[macro_use]
// extern crate stdweb;
extern crate tiled;

mod constant;
mod direction;
mod game_layer;
mod game_map;
mod grid;
mod player;
mod player_state;

use game_map::GameMap;
use player::Player;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

struct RoboRex {
    time: f64,
    player: Player,
    game_map: Asset<GameMap>,
    font: Asset<Font>,
}

impl State for RoboRex {
    fn new() -> Result<RoboRex> {
        let game_map = Asset::new(GameMap::load("resources/tiled/level.tmx"));
        let font = Asset::new(Font::load("resources/fonts/slkscr.ttf"));

        let roborex = RoboRex {
            time: 0.,
            player: Player::new(),
            game_map,
            font,
        };

        Ok(roborex)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.time += window.update_rate();
        let player = &mut self.player;
        self.game_map.execute(|game_map| {
            player.update(window, game_map)?;
            Ok(())
        })?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let player = &mut self.player;
        self.game_map.execute(|game_map| {
            game_map.draw(window)?;
            player.draw(window)?;
            Ok(())
        })?;

        self.font.execute(|font| {
            let big = FontStyle::new(72.0, Color::WHITE);
            let normal = FontStyle::new(42.0, Color::WHITE);
            let word = font.render("Apple", &big)?;
            let instruction = font.render("Find all the letters for the word:", &normal)?;
            window.draw_ex(
                &instruction.area().with_center((1920 / 2, 30)),
                Img(&instruction),
                Transform::scale(Vector::new(1, 1)),
                4,
            );
            window.draw_ex(
                &word.area().with_center((1920 / 2, 90)),
                Img(&word),
                Transform::scale(Vector::new(1, 1)),
                4,
            );
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<RoboRex>("RoboRex", Vector::new(1920, 1080), Settings::default());
}
