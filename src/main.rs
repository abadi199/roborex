#[macro_use]
extern crate lazy_static;

extern crate futures;
extern crate nalgebra;
extern crate quicksilver;

// #[macro_use]
// extern crate stdweb;
extern crate tiled;

mod collectible;
mod constant;
mod direction;
mod game_layer;
mod game_map;
mod grid;
mod instruction;
mod player;
mod player_state;

use collectible::Collectible;
use game_map::GameMap;
use instruction::CanCollect;
use instruction::Instruction;
use player::Player;
use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

struct RoboRex {
    time: f64,
    player: Player,
    game_map: Asset<GameMap>,
    instruction: Instruction,
    collectible: Vec<Collectible>,
}

impl State for RoboRex {
    fn new() -> Result<RoboRex> {
        let game_map = Asset::new(GameMap::load("resources/tiled/level.tmx"));
        let instruction = Instruction::new("APPLE".to_string());
        let collectible = vec![
            Collectible::new('A', (5, 7)),
            Collectible::new('P', (10, 12)),
            Collectible::new('P', (18, 7)),
            Collectible::new('L', (17, 11)),
            Collectible::new('E', (22, 11)),
        ];

        let roborex = RoboRex {
            time: 0.,
            player: Player::new(),
            game_map,
            instruction,
            collectible,
        };

        Ok(roborex)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.time += window.update_rate();
        let player = &mut self.player;
        let collectibles = &mut self.collectible;
        let instruction = &mut self.instruction;
        self.game_map.execute(|game_map| {
            player.update(window, game_map)?;
            for collectible in collectibles.into_iter() {
                if collectible.collide_with(player) {
                    match instruction.collect(collectible.letter) {
                        CanCollect::Yes => collectible.collect(),
                        CanCollect::No => {}
                    }
                }
            }
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

        self.instruction.draw(window)?;

        let collectible = &mut self.collectible;
        for c in collectible.into_iter() {
            c.draw(window)?;
        }

        Ok(())
    }
}

fn main() {
    run::<RoboRex>(
        "RoboRex",
        Vector::new(constant::WINDOW_WIDTH, constant::WINDOW_HEIGHT),
        Settings::default(),
    );
}
