use quicksilver::{
    graphics::{Background::Img, Image},
    input::{ButtonState, Key, MouseButton},
    lifecycle::{Asset, Window},
    Result,
};

pub struct Splash {
    state: State,
    image: Asset<Image>,
}

enum State {
    Waiting,
    Clicked,
}

impl Splash {
    pub fn new() -> Self {
        let image = Asset::new(Image::load("resources/images/splash.png"));
        let state = State::Waiting;

        Self { state, image }
    }

    pub fn is_clicked(&self) -> bool {
        match self.state {
            State::Waiting => false,
            State::Clicked => true,
        }
    }

    pub fn update(&mut self, window: &mut Window) -> Result<()> {
        if let State::Clicked = self.state {
            return Ok(());
        }

        if window.mouse()[MouseButton::Left] == ButtonState::Released {
            self.state = State::Clicked;
        }

        if window.keyboard()[Key::Return].is_down() {
            self.state = State::Clicked;
        }

        Ok(())
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.image.execute(|image| {
            window.draw(&image.area(), Img(&image));
            Ok(())
        })?;
        Ok(())
    }
}
