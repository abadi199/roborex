use constant::{TILE_HEIGHT, TILE_WIDTH, WALKING_DURATION};
use direction::Direction;
use player_state::PlayerState;
use quicksilver::geom::{Rectangle, Vector};

pub enum Grid {
    Path,
    NonPath,
}

impl Grid {
    pub fn to_rectangle(x: u32, y: u32) -> Rectangle {
        Rectangle::new(
            ((x * TILE_WIDTH * 2) + 8, (y * TILE_HEIGHT * 2) + 8),
            (TILE_WIDTH, TILE_HEIGHT),
        )
    }

    pub fn to_coordinate((x, y): (u32, u32)) -> Vector {
        Vector::new((x * TILE_WIDTH * 2) + 8, (y * TILE_HEIGHT * 2) + 8)
    }

    pub fn to_player_coordinate(state: &PlayerState, (x, y): (u32, u32)) -> Vector {
        let (delta_x, delta_y) = match state {
            PlayerState::Walking {
                direction: Direction::Right,
                timer,
                ..
            } => (((WALKING_DURATION - timer) / WALKING_DURATION) * 32., 0.),
            PlayerState::Walking {
                direction: Direction::Left,
                timer,
                ..
            } => (((WALKING_DURATION - timer) / WALKING_DURATION) * -32., 0.),
            PlayerState::Walking {
                direction: Direction::Up,
                timer,
                ..
            } => (0., ((WALKING_DURATION - timer) / WALKING_DURATION) * -32.),
            PlayerState::Walking {
                direction: Direction::Down,
                timer,
                ..
            } => (0., ((WALKING_DURATION - timer) / WALKING_DURATION) * 32.),
            _ => (0., 0.),
        };

        Vector::new(
            (((x * TILE_WIDTH * 2) + 40) as i32 + delta_x as i32) as u32,
            (((y * TILE_HEIGHT * 2) + 24) as i32 + delta_y as i32) as u32,
        )
    }
}
