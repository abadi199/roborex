use constant::COLLECTIBLE_Z;
use grid::Grid;
use player::Player;
use primitive::Position;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    Result,
};

pub struct Collectible {
    pub status: Status,
    pub position: Position,
    pub letter: char,
    pub font: Asset<Font>,
}

impl Collectible {
    pub fn new(letter: char, position: Position) -> Self {
        let font = Asset::new(Font::load("resources/fonts/slkscr.ttf"));
        Collectible {
            status: Status::NotCollected,
            position,
            letter,
            font,
        }
    }

    pub fn collect(&mut self) {
        self.status = Status::Collected;
    }

    pub fn collide_with(&self, player: &Player) -> bool {
        self.status == Status::NotCollected && self.position == player.position
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        if let Status::Collected = self.status {
            return Ok(());
        }

        let letter = &self.letter;
        let position = &self.position;
        self.font.execute(|font| {
            let normal = FontStyle::new(24.0, Color::WHITE);
            let word_text = font.render(&letter.to_string(), &normal)?;
            let rectangle = Grid::to_collectible_coordinate(position);

            window.draw_ex(
                &word_text.area().with_center(rectangle.pos),
                Img(&word_text),
                Transform::scale(Vector::new(1, 1)),
                COLLECTIBLE_Z,
            );
            Ok(())
        })?;

        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub enum Status {
    NotCollected,
    Collected,
}
