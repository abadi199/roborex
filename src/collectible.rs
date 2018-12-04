use constant::COLLECTIBLE_Z;
use grid::Grid;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    Result,
};

pub struct Collectible {
    pub status: Status,
    pub position: (u32, u32),
    pub character: char,
    pub font: Asset<Font>,
}

impl Collectible {
    pub fn new(character: char, position: (u32, u32)) -> Self {
        let font = Asset::new(Font::load("resources/fonts/slkscr.ttf"));
        Collectible {
            status: Status::NotCollected,
            position,
            character,
            font,
        }
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let character = &self.character;
        let position = &self.position;
        self.font.execute(|font| {
            let normal = FontStyle::new(24.0, Color::WHITE);
            let word_text = font.render(&format!("{}", character), &normal)?;
            let rectangle = Grid::to_collectible_coordinate(position.0, position.1);

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

pub enum Status {
    NotCollected,
    Collected,
}
