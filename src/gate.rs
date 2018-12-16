use constant;
use grid::Grid;
use primitive::{Dimension, Position};
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, Image},
    lifecycle::Window,
    Result,
};

const GATE_INDEX: u32 = 214;

pub struct Gate {
    position: Position,
    state: State,
    gate_rec: Rectangle,
}

#[derive(PartialEq)]
pub enum State {
    Opened,
    Closed,
}

impl Gate {
    pub fn new(position: Position, tile: Dimension, image: Dimension) -> Self {
        let gate_rec = Self::to_rectangle(GATE_INDEX, &tile, &image);
        Gate {
            position,
            state: State::Closed,
            gate_rec,
        }
    }

    pub fn open(&mut self) {
        self.state = State::Opened;
    }

    pub fn is_gate(&self, x: u32, y: u32) -> bool {
        self.is_close()
            && self.position.x == x
            && (self.position.y == y || (self.position.y + 1) == y || (self.position.y + 2) == y)
    }

    fn is_close(&self) -> bool {
        match self.state {
            State::Opened => false,
            State::Closed => true,
        }
    }

    pub fn draw(&mut self, window: &mut Window, tileset: &Image) -> Result<()> {
        if self.state == State::Closed {
            let draw_rec = Grid::to_rectangle(self.position.x, self.position.y);
            let scale = Transform::scale(Vector::new(2, 2));
            window.draw_ex(
                &draw_rec,
                Img(&tileset.subimage(self.gate_rec)),
                scale,
                constant::GATE_Z,
            );

            let draw_rec = Grid::to_rectangle(self.position.x, self.position.y + 1);
            window.draw_ex(
                &draw_rec,
                Img(&tileset.subimage(self.gate_rec)),
                scale,
                constant::GATE_Z,
            );

            let draw_rec = Grid::to_rectangle(self.position.x, self.position.y + 2);
            window.draw_ex(
                &draw_rec,
                Img(&tileset.subimage(self.gate_rec)),
                scale,
                constant::GATE_Z,
            );
        }
        Ok(())
    }

    fn to_rectangle(tile_index: u32, tile: &Dimension, image: &Dimension) -> Rectangle {
        Rectangle::new(
            (
                ((tile_index % (image.width / tile.width) * tile.width) - tile.width) as f32,
                (tile_index / (image.width / tile.height) * tile.height) as f32,
            ),
            (tile.width as f32, tile.height as f32),
        )
    }
}
