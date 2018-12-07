use constant;
use quicksilver::{
    geom::{Shape, Transform, Vector},
    graphics::{Background::Img, Color, Font, FontStyle},
    lifecycle::{Asset, Window},
    Result,
};

lazy_static! {
    static ref BIG: FontStyle = FontStyle::new(42.0, Color::WHITE);
    static ref NORMAL: FontStyle = FontStyle::new(24.0, Color::WHITE);
}

pub struct Instruction {
    pub answer: Vec<Answered>,
    pub font: Asset<Font>,
}

pub enum Answered {
    No(char),
    Yes(char),
}

impl Answered {
    pub fn to_rendered_char(&self) -> char {
        match *self {
            Answered::Yes(letter) => letter,
            Answered::No(_) => '_',
        }
    }
}

pub enum CanCollect {
    Yes,
    No,
}

impl Instruction {
    pub fn new(word: String) -> Self {
        let font = Asset::new(Font::load("resources/fonts/slkscr.ttf"));
        let answer = word.chars().map(|letter| Answered::No(letter)).collect();
        Instruction { font, answer }
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let answer = &self.answer;
        self.font.execute(|font| {
            let instruction_text = font.render("Collect all the letters for the word:", &NORMAL)?;
            let instruction_height = instruction_text.area().height() as u32;
            let answer_to_render: String = answer
                .iter()
                .map(|letter| letter.to_rendered_char())
                .collect();

            let answer_text = font.render(&answer_to_render, &BIG)?;
            let answer_height = answer_text.area().height() as u32;
            window.draw_ex(
                &instruction_text.area().with_center((
                    constant::WINDOW_WIDTH / 2,
                    constant::WINDOW_HEIGHT - (instruction_height + answer_height),
                )),
                Img(&instruction_text),
                Transform::scale(Vector::new(1, 1)),
                4,
            );

            window.draw_ex(
                &answer_text.area().with_center((
                    constant::WINDOW_WIDTH / 2,
                    constant::WINDOW_HEIGHT - (answer_height * 2 / 3),
                )),
                Img(&answer_text),
                Transform::scale(Vector::new(1, 1)),
                4,
            );
            Ok(())
        })?;

        Ok(())
    }

    pub fn collect(&mut self, letter: char) -> CanCollect {
        let index = self.find(letter);
        match index {
            None => CanCollect::No,
            Some(index) => {
                self.answer[index] = Answered::Yes(letter);
                return CanCollect::Yes;
            }
        }
    }

    fn find(&self, letter: char) -> Option<usize> {
        for (i, answer) in self.answer.iter().enumerate() {
            if let Answered::No(answer_letter) = answer {
                if letter == *answer_letter {
                    return Some(i);
                } else {
                    return None;
                }
            }
        }

        None
    }
}
