use constant;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    Result,
};

pub struct Instruction {
    pub word: String,
    pub font: Asset<Font>,
}

impl Instruction {
    pub fn new(word: String) -> Self {
        let font = Asset::new(Font::load("resources/fonts/slkscr.ttf"));
        Instruction { word, font }
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let word = &self.word;
        self.font.execute(|font| {
            let big = FontStyle::new(42.0, Color::WHITE);
            let normal = FontStyle::new(24.0, Color::WHITE);
            let word_text = font.render(word, &big)?;
            let instruction_text = font.render("Collect all the letters for the word:", &normal)?;
            let instruction_height = instruction_text.area().height() as u32;
            let word_height = word_text.area().height() as u32;
            window.draw_ex(
                &instruction_text.area().with_center((
                    constant::WINDOW_WIDTH / 2,
                    constant::WINDOW_HEIGHT - (instruction_height + word_height),
                )),
                Img(&instruction_text),
                Transform::scale(Vector::new(1, 1)),
                4,
            );
            window.draw_ex(
                &word_text.area().with_center((
                    constant::WINDOW_WIDTH / 2,
                    constant::WINDOW_HEIGHT - word_height,
                )),
                Img(&word_text),
                Transform::scale(Vector::new(1, 1)),
                4,
            );
            Ok(())
        })?;

        Ok(())
    }
}
