use constant::{GRID_HEIGHT, GRID_WIDTH, TILE_HEIGHT, TILE_WIDTH, WALKING_DURATION};
use direction::Direction;
use player_state::PlayerState;
use quicksilver::geom::{Rectangle, Vector};

#[derive(Debug, Clone, Copy)]
pub enum Grid {
    Path,
    NonPath,
    Empty,
}

const PLAYER_X_OFFSET: u32 = TILE_WIDTH * 2;
const PLAYER_Y_OFFSET: u32 = TILE_HEIGHT + (TILE_HEIGHT / 2) + (TILE_HEIGHT * 8);
const GRID_X_OFFSET: u32 = TILE_WIDTH / 2;
const GRID_Y_OFFSET: u32 = TILE_HEIGHT / 2 + (TILE_HEIGHT * 8);

impl Grid {
    pub fn to_rectangle(x: u32, y: u32) -> Rectangle {
        Rectangle::new(
            (
                (x * GRID_WIDTH) + GRID_X_OFFSET,
                (y * GRID_HEIGHT) + GRID_Y_OFFSET,
            ),
            (TILE_WIDTH, TILE_HEIGHT),
        )
    }

    pub fn from_coordinate(coordinate: Vector) -> (u32, u32) {
        (
            ((coordinate.x - GRID_X_OFFSET as f32) / GRID_WIDTH as f32) as u32,
            ((coordinate.y - GRID_Y_OFFSET as f32) / GRID_HEIGHT as f32) as u32,
        )
    }

    pub fn to_player_coordinate(state: &PlayerState, (x, y): (u32, u32)) -> Vector {
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
            (((x * GRID_WIDTH) + PLAYER_X_OFFSET) as i32 + delta_x as i32) as u32,
            (((y * GRID_HEIGHT) + PLAYER_Y_OFFSET) as i32 + delta_y as i32) as u32,
        )
    }
}
