use futures::{future, Future};
use game_layer::GameLayer;
use grid::Grid;
use quicksilver::{
    geom::Rectangle,
    graphics::Image,
    lifecycle::{Asset, Window},
    load_file, Error, Result,
};
use std::cmp::max;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tiled;

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

pub struct GameMap {
    layers: Vec<GameLayer>,
    grid: GridMap,
}

type GridMap = Vec<Vec<Grid>>;

impl GameMap {
    pub fn load<'a, P: 'static + AsRef<Path>>(
        path: P,
    ) -> impl Future<Item = GameMap, Error = Error> {
        load_file(PathBuf::from(path.as_ref()))
            .map(|data| Self::from_bytes(data.as_slice()))
            .and_then(future::result)
    }

    pub fn from_bytes(raw: &[u8]) -> Result<GameMap> {
        let map = tiled::parse(raw)
            .map_err(|_| Error::ContextError("Error loading level".to_string()))?;
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
            }).collect();
        let grid: GridMap = Self::to_grid(map.layers);
        Ok(GameMap { layers, grid })
    }

    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let len = self.layers.len();
        for i in 0..len {
            self.layers[i].draw(window)?;
        }

        Ok(())
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
                            }).collect()
                    }).collect()
            }).fold(None, |a, b| match a {
                None => Some(b),
                Some(a) => Some(Self::join(a, b)),
            }).unwrap()
    }

    fn join(a: GridMap, b: GridMap) -> GridMap {
        let max_y = max(a.len(), b.len());
        let new_grid: GridMap = (0..max_y)
            .map(|y| {
                let row_a = a.get(y);
                let row_b = b.get(y);
                let max_x = max(
                    row_a.map(|v| v.len()).unwrap_or(0),
                    row_b.map(|v| v.len()).unwrap_or(0),
                );
                (0..max_x)
                    .map(|x| {
                        let grid_a = row_a.and_then(|v| v.get(x)).unwrap_or(&Grid::Empty);
                        let grid_b = row_b.and_then(|v| v.get(x)).unwrap_or(&Grid::Empty);
                        match (grid_a, grid_b) {
                            (Grid::Path, Grid::Empty) => Grid::Path,
                            (Grid::Empty, Grid::Empty) => Grid::Empty,
                            (Grid::Empty, Grid::Path) => Grid::Path,
                            (Grid::Path, Grid::Path) => Grid::Path,
                            (_, Grid::NonPath) => Grid::NonPath,
                            (Grid::NonPath, _) => Grid::NonPath,
                        }
                    }).collect()
            }).collect();

        new_grid
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
            }).collect();
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
