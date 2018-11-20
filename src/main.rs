// Draw some multi-colored geometry to the screen
extern crate nalgebra;
extern crate quicksilver;
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
    lifecycle::{run, Settings, State, Window},
    Result,
};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::parse;

// A unit struct that we're going to use to run the Quicksilver functions
struct RoboRex {
    time: f64,
    player: Player,
    game_map: GameMap,
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
