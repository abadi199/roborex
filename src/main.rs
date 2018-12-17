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
mod gate;
mod grid;
mod level;
mod player;
mod player_state;
mod primitive;
mod puzzle;
mod splash;

use level::Level;
use player::Player;
use quicksilver::{
    geom::Vector,
    graphics::Color,
    lifecycle::{run, Settings, State, Window},
    Result,
};
use splash::Splash;

struct RoboRex {
    time: f64,
    player: Player,
    level: Level,
    state: GameState,
    splash: Splash,
}

enum GameState {
    Splash,
    Playing,
}

impl RoboRex {
    fn start_level(&mut self, level: Level) {
        self.level = level;
        self.player.position = self.level.start_position.clone();
    }

    fn update_playing(&mut self, window: &mut Window) -> Result<()> {
        self.time += window.update_rate();
        self.level.update(window, &mut self.player)?;

        if self.level.passing_the_gate(&self.player) {
            let next_level = self.level.next_level();
            match next_level {
                Some(level) => self.start_level(level),
                None => panic!("Finish level not implemented yet"),
            }
        }

        Ok(())
    }

    fn update_splash(&mut self, window: &mut Window) -> Result<()> {
        self.splash.update(window)?;
        if (self.splash.is_clicked()) {
            self.state = GameState::Playing;
        }

        Ok(())
    }

    fn draw_playing(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.level.draw(window, &mut self.player)?;
        Ok(())
    }

    fn draw_splash(&mut self, window: &mut Window) -> Result<()> {
        self.splash.draw(window)?;
        Ok(())
    }
}

impl State for RoboRex {
    fn new() -> Result<RoboRex> {
        let splash = Splash::new();
        let level = Level::start();
        let mut player = Player::new();
        player.position = level.start_position.clone();
        let roborex = RoboRex {
            time: 0.,
            player,
            level,
            splash,
            state: GameState::Splash,
        };

        Ok(roborex)
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        match self.state {
            GameState::Splash => self.update_splash(window),
            GameState::Playing => self.update_playing(window),
        }
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        match self.state {
            GameState::Splash => self.draw_splash(window),
            GameState::Playing => self.draw_playing(window),
        }
    }
}

fn main() {
    run::<RoboRex>(
        "RoboRex",
        Vector::new(constant::WINDOW_WIDTH, constant::WINDOW_HEIGHT),
        Settings::default(),
    );
}
