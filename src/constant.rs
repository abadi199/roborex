pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGHT: u32 = 16;
pub const WALKING_DURATION: f64 = 300.;
pub const SCALING_FACTOR: f32 = 2.;
pub const GRID_WIDTH: u32 = (TILE_WIDTH as f32 * SCALING_FACTOR) as u32;
pub const GRID_HEIGHT: u32 = (TILE_HEIGHT as f32 * SCALING_FACTOR) as u32;
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

pub const COLLECTIBLE_Z: u32 = 10;
pub const GATE_Z: u32 = 11;
pub const PLAYER_Z: u32 = 12;
