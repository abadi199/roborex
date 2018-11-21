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
        let grid: GridMap = Self::to_grid(map.layers);
        for row in grid.iter() {
            println!("{:?}", row);
        }
        GameMap { layers, grid }
    }

    pub fn draw(&mut self, window: &mut Window) {
        let len = self.layers.len();
        for i in 0..len {
            self.layers[i].draw(window);
        }
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
            Grid::Empty => false,
        }
    }

    fn to_grid(layers: Vec<tiled::Layer>) -> GridMap {
        layers
            .iter()
            .map(|layer| {
                layer
                    .tiles
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|tile| {
                                if tile == &0 {
                                    return Grid::Empty;
                                }
                                if PATH_SET.contains(tile) {
                                    return Grid::Path;
                                }
                                Grid::NonPath
                            })
                            .collect()
                    })
                    .collect()
            })
            .fold(None, |a, b| match a {
                None => Some(b),
                Some(a) => Some(Self::join(a, b)),
            })
            .unwrap()
    }

    fn join(a: GridMap, b: GridMap) -> GridMap {
        a
        //     a.iter()
        //         .map(|row| row.iter().map(|grid| grid.clone()).collect())
        //         .collect()
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
                if layer.name == "Fence" {
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
