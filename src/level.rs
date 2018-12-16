use collectible::Collectible;
use game_map::GameMap;
use gate::Gate;
use player::Player;
use primitive::Position;
use puzzle::{CanCollect, Puzzle};
use quicksilver::{
    lifecycle::{Asset, Window},
    Result,
};

pub struct Level {
    index: u32,
    pub start_position: (u32, u32),
    game_map: Asset<GameMap>,
    puzzle: Puzzle,
    collectible: Vec<Collectible>,
}

pub enum Solved {
    Yes,
    No,
}

impl Level {
    pub fn start() -> Level {
        Self::level1()
    }

    fn new(level: u32) -> Option<Level> {
        match level {
            0 => Some(Self::level1()),
            1 => Some(Self::level2()),
            _ => None,
        }
    }

    pub fn next_level(&self) -> Option<Level> {
        Self::new(self.index + 1)
    }

    fn level2() -> Self {
        let game_map = Asset::new(GameMap::load(
            "resources/tiled/level2.tmx",
            Position::new(0, 0),
        ));
        let puzzle = Puzzle::new("JONATHAN".to_string());
        let collectible = vec![
            Collectible::new('J', (5, 14)),
            Collectible::new('O', (7, 14)),
            Collectible::new('N', (8, 14)),
            Collectible::new('A', (15, 14)),
            Collectible::new('T', (18, 14)),
            Collectible::new('H', (20, 14)),
            Collectible::new('A', (21, 14)),
            Collectible::new('N', (22, 14)),
        ];

        Level {
            index: 1,
            start_position: (0, 14),
            game_map,
            puzzle,
            collectible,
        }
    }

    fn level1() -> Self {
        let game_map = Asset::new(GameMap::load(
            "resources/tiled/level1.tmx",
            Position::new(24, 13),
        ));
        let puzzle = Puzzle::new("APPLE".to_string());
        let collectible = vec![
            Collectible::new('A', (5, 7)),
            Collectible::new('P', (10, 12)),
            Collectible::new('P', (18, 7)),
            Collectible::new('L', (17, 11)),
            Collectible::new('E', (22, 11)),
        ];

        Level {
            index: 0,
            start_position: (0, 14),
            game_map,
            puzzle,
            collectible,
        }
    }

    pub fn update(&mut self, window: &mut Window, player: &mut Player) -> Result<Solved> {
        let collectibles = &mut self.collectible;
        let puzzle = &mut self.puzzle;
        self.game_map.execute(|game_map| {
            player.update(window, game_map)?;
            for collectible in collectibles.into_iter() {
                if collectible.collide_with(player) {
                    match puzzle.collect(collectible.letter) {
                        CanCollect::Yes => collectible.collect(),
                        CanCollect::No => {}
                    }
                }
            }
            puzzle.update(window)?;
            if puzzle.is_solved() {
                game_map.open_gate();
            }

            Ok(())
        })?;

        match puzzle.is_solved() {
            true => Ok(Solved::Yes),
            false => Ok(Solved::No),
        }
    }

    pub fn draw(&mut self, window: &mut Window, player: &mut Player) -> Result<()> {
        self.game_map.execute(|game_map| {
            game_map.draw(window)?;
            player.draw(window)?;
            Ok(())
        })?;

        self.puzzle.draw(window)?;

        let collectible = &mut self.collectible;
        for c in collectible.into_iter() {
            c.draw(window)?;
        }

        Ok(())
    }
}
