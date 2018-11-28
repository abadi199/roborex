use grid::Grid;
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Background::Img, Image},
    lifecycle::{Asset, Window},
    Result,
};

pub struct GameLayer {
    pub rectangles: Vec<Vec<Option<Rectangle>>>,
    pub image: Asset<Image>,
}

impl GameLayer {
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let rectangles = &self.rectangles;
        self.image.execute(|image| {
            for (y, row) in rectangles.iter().enumerate() {
                for (x, col) in row.iter().enumerate() {
                    if let Some(rec) = col {
                        let draw_rec = Grid::to_rectangle(x as u32, y as u32);
                        window.draw_ex(
                            &draw_rec,
                            Img(&image.subimage(*rec)),
                            Transform::scale(Vector::new(2, 2)),
                            1,
                        );
                    }
                }
            }
            Ok(())
        })?;

        Ok(())
    }
}
