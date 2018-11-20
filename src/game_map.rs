use game_layer::GameLayer;
use grid::Grid;
use quicksilver::{
    geom::Rectangle,
    graphics::Image,
    lifecycle::{Asset, Window},
};
use tiled;

pub struct GameMap {
    layers: Vec<GameLayer>,
    grid: Vec<Vec<Grid>>,
}

impl GameMap {
    pub fn load(map: tiled::Map) -> Self {
        let layers: Vec<GameLayer> = map
            .layers
            .iter()
            .map(|layer| {
                Self::to_game_layer(
                    &layer,
                    map.tile_width,
                    map.tile_height,
                    &map.tilesets[0].images[0],
                )
            })
            .collect();
        GameMap {
            layers,
            grid: vec![],
        }
    }

    pub fn draw(&mut self, window: &mut Window) {
        self.layers[0].draw(window);
        self.layers[1].draw(window);
        self.layers[2].draw(window);
    }

    fn to_game_layer(
        layer: &tiled::Layer,
        tile_width: u32,
        tile_height: u32,
        image: &tiled::Image,
    ) -> GameLayer {
        let rectangles = layer
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile: &u32| Self::to_rectangle(*tile, tile_width, tile_height, image))
                    .collect()
            })
            .collect();
        let image = Asset::new(Image::load(format!("resources/tiled/{}", image.source)));
        GameLayer { rectangles, image }
    }

    fn to_rectangle(
        tile: u32,
        tile_width: u32,
        tile_height: u32,
        image: &tiled::Image,
    ) -> Option<Rectangle> {
        match tile {
            0 => None,
            _ => Some(Rectangle::new(
                (
                    ((tile % (image.width as u32 / tile_width) * tile_width) - tile_width) as f32,
                    (tile / (image.width as u32 / tile_height) * tile_height) as f32,
                ),
                (tile_width as f32, tile_height as f32),
            )),
        }
    }
}
