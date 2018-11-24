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
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Asset, Settings, State, Window},
    load_file, Result,
};


use std::path::{Path, PathBuf};
use tiled::parse;
use std::str;

struct RoboRex {
    time: f64,
    player: Player,
    game_map: Asset<GameMap>,
}

impl State for RoboRex {
    fn new() -> Result<RoboRex> {
        let game_map_asset = Asset::new(GameMap::load("resources/tiled/level.tmx"));

        let roborex = RoboRex {
            time: 0.,
            player: Player::new(),
            game_map: game_map_asset,
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
        window.clear(Color::WHITE)?;
        let player = &mut self.player;
        self.game_map.execute(|game_map| {
            game_map.draw(window)?;
            player.draw(window)?;
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<RoboRex>("RoboRex", Vector::new(512, 512), Settings::default());
}
