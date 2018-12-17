use constant::{GRID_HEIGHT, GRID_WIDTH, TILE_HEIGHT, TILE_WIDTH, WALKING_DURATION};
use direction::Direction;
use player_state::PlayerState;
use primitive::Position;
use quicksilver::geom::{Rectangle, Vector};
use std::fmt;

#[derive(Clone, Copy)]
pub enum Grid {
    Path,
    NonPath,
    Empty,
}

const COLLECTIBLE_X_OFFSET: u32 = TILE_WIDTH * 2;
const COLLECTIBLE_Y_OFFSET: u32 = TILE_HEIGHT * 2;
const PLAYER_X_OFFSET: u32 = TILE_WIDTH * 2;
const PLAYER_Y_OFFSET: u32 = TILE_HEIGHT + (TILE_HEIGHT / 2);
const GRID_X_OFFSET: u32 = TILE_WIDTH / 2;
const GRID_Y_OFFSET: u32 = TILE_HEIGHT / 2;

impl Grid {
    pub fn to_collectible_coordinate(position: &Position) -> Rectangle {
        Rectangle::new(
            (
                (position.x * GRID_WIDTH) + COLLECTIBLE_X_OFFSET,
                (position.y * GRID_HEIGHT) + COLLECTIBLE_Y_OFFSET,
            ),
            (TILE_WIDTH, TILE_HEIGHT),
        )
    }

    pub fn to_rectangle(position: &Position) -> Rectangle {
        Rectangle::new(
            (
                (position.x * GRID_WIDTH) + GRID_X_OFFSET,
                (position.y * GRID_HEIGHT) + GRID_Y_OFFSET,
            ),
            (TILE_WIDTH, TILE_HEIGHT),
        )
    }

    pub fn from_coordinate(coordinate: Vector) -> Position {
        Position::new(
            ((coordinate.x - GRID_X_OFFSET as f32) / GRID_WIDTH as f32) as u32,
            ((coordinate.y - GRID_Y_OFFSET as f32) / GRID_HEIGHT as f32) as u32,
        )
    }

    pub fn to_player_coordinate(state: &PlayerState, position: &Position) -> Vector {
        let (delta_x, delta_y) = match state {
            PlayerState::Walking {
                direction: Direction::Right,
                timer,
                ..
            } => (
                ((WALKING_DURATION - timer) / WALKING_DURATION) * GRID_WIDTH as f64,
                0.,
            ),
            PlayerState::Walking {
                direction: Direction::Left,
                timer,
                ..
            } => (
                ((WALKING_DURATION - timer) / WALKING_DURATION) * -(GRID_WIDTH as f64),
                0.,
            ),
            PlayerState::Walking {
                direction: Direction::Up,
                timer,
                ..
            } => (
                0.,
                ((WALKING_DURATION - timer) / WALKING_DURATION) * -(GRID_HEIGHT as f64),
            ),
            PlayerState::Walking {
                direction: Direction::Down,
                timer,
                ..
            } => (
                0.,
                ((WALKING_DURATION - timer) / WALKING_DURATION) * GRID_HEIGHT as f64,
            ),
            _ => (0., 0.),
        };

        Vector::new(
            (((position.x * GRID_WIDTH) + PLAYER_X_OFFSET) as i32 + delta_x as i32) as u32,
            (((position.y * GRID_HEIGHT) + PLAYER_Y_OFFSET) as i32 + delta_y as i32) as u32,
        )
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grid::Empty => write!(f, " "),
            Grid::NonPath => write!(f, "#"),
            Grid::Path => write!(f, "="),
        }
    }
}
