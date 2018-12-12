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
mod level;
mod player;
mod player_state;
mod puzzle;

use level::Level;
use player::Player;
use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Settings, State, Window},
    Result,
};

struct RoboRex {
    time: f64,
    player: Player,
    level: Level,
}

impl RoboRex {
    fn start_level(&mut self, level: Level) {
        self.level = level;
        self.player.position = self.level.start_position;
    }
}

impl State for RoboRex {
    fn new() -> Result<RoboRex> {
        let level = Level::start();
        let mut player = Player::new();
        player.position = level.start_position;
        let roborex = RoboRex {
            time: 0.,
            player,
            level,
        };

        Ok(roborex)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.time += window.update_rate();
        match self.level.update(window, &mut self.player)? {
            level::Solved::Yes => {
                println!("Level Solved!");
                let next_level = self.level.next_level();
                match next_level {
                    Some(level) => self.start_level(level),
                    None => panic!("Finish level not implemented yet"),
                }
            }
            level::Solved::No => {}
        };

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.level.draw(window, &mut self.player)?;
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
