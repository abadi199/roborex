use game_layer::GameLayer;
use grid::Grid;
use quicksilver::{
    geom::Rectangle,
    graphics::Image,
    lifecycle::{Asset, Window},
};
use std::collections::HashSet;
use tiled;

pub struct GameMap {
    layers: Vec<GameLayer>,
    grid: GridMap,
}

lazy_static! {
    static ref PATH_SET: HashSet<u32> = {
        let mut m = HashSet::new();
        m.insert(197);
        m.insert(196);
        m.insert(221);
        m.insert(271);
        m.insert(246);
        m.insert(269);
        m.insert(268);
        m.insert(270);
        m
    };
}

type GridMap = Vec<Vec<Grid>>;

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
        let grid: GridMap = Self::to_grid(&map.layers[0]);
        for row in grid.iter() {
            println!("{:?}", row);
        }
        GameMap { layers, grid }
    }

    pub fn draw(&mut self, window: &mut Window) {
        self.layers[0].draw(window);
        self.layers[1].draw(window);
        self.layers[2].draw(window);
    }

    pub fn can_walk_to(&self, (x, y): (u32, u32)) -> bool {
        let x = x as usize;
        let y = y as usize;
        if self.grid.len() <= y || self.grid[y].len() <= x {
            return false;
        }

        match self.grid[y][x] {
            Grid::Path => true,
            Grid::NonPath => false,
        }
    }

    fn to_grid(layer: &tiled::Layer) -> GridMap {
        layer
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| {
                        if PATH_SET.contains(tile) {
                            return Grid::Path;
                        }
                        Grid::NonPath
                    })
                    .collect()
            })
            .collect()
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
                if layer.name == "Ground" {
                    println!("{:?}", row);
                }
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
