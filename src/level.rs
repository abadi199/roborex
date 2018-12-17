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
    pub start_position: Position,
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
            Position::new(24, 14),
        ));
        let puzzle = Puzzle::new("JONATHAN".to_string());
        let collectible = vec![
            Collectible::new('J', Position::new(5, 14)),
            Collectible::new('O', Position::new(7, 14)),
            Collectible::new('N', Position::new(8, 14)),
            Collectible::new('A', Position::new(15, 14)),
            Collectible::new('T', Position::new(18, 14)),
            Collectible::new('H', Position::new(20, 14)),
            Collectible::new('A', Position::new(21, 14)),
            Collectible::new('N', Position::new(22, 14)),
        ];

        Level {
            index: 1,
            start_position: Position::new(0, 14),
            game_map,
            puzzle,
            collectible,
        }
    }

    fn level1() -> Self {
        let game_map = Asset::new(GameMap::load(
            "resources/tiled/level1.tmx",
            Position::new(24, 14),
        ));
        let puzzle = Puzzle::new("APPLE".to_string());
        let collectible = vec![
            Collectible::new('A', Position::new(5, 7)),
            Collectible::new('P', Position::new(10, 12)),
            Collectible::new('P', Position::new(18, 7)),
            Collectible::new('L', Position::new(17, 11)),
            Collectible::new('E', Position::new(22, 11)),
        ];

        Level {
            index: 0,
            start_position: Position::new(0, 14),
            game_map,
            puzzle,
            collectible,
        }
    }

    pub fn update(&mut self, window: &mut Window, player: &mut Player) -> Result<()> {
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

        Ok(())
    }

    pub fn passing_the_gate(&mut self, player: &Player) -> bool {
        let mut passing_the_gate = false;
        self.game_map.execute(|game_map| {
            passing_the_gate = game_map.gate_position() == &player.position;
            Ok(())
        });

        passing_the_gate
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
