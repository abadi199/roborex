use grid::Grid;
use primitive::Position;
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, Image},
    lifecycle::{Asset, Window},
    Result,
};
use std::fmt;

pub struct GameLayer {
    pub name: String,
    pub tiles: Vec<Vec<u32>>,
    pub rectangles: Vec<Vec<Option<Rectangle>>>,
}

impl GameLayer {
    pub fn draw(&mut self, window: &mut Window, tileset: &Image) -> Result<()> {
        let rectangles = &self.rectangles;

        for (y, row) in rectangles.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if let Some(rec) = col {
                    let draw_rec = Grid::to_rectangle(&Position::new(x as u32, y as u32));
                    window.draw_ex(
                        &draw_rec,
                        Img(&tileset.subimage(*rec)),
                        Transform::scale(Vector::new(2, 2)),
                        1,
                    );
                }
            }
        }

        Ok(())
    }
}

impl fmt::Debug for GameLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        for row in self.tiles.iter() {
            writeln!(f, "{:?}", row)?;
        }

        Ok(())
    }
}
