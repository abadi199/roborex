#[derive(PartialEq)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Dimension { width, height }
    }
}

#[derive(PartialEq, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }

    pub fn add(&self, x: i32, y: i32) -> Self {
        Position {
            x: (self.x as i32 + x) as u32,
            y: (self.y as i32 + y) as u32,
        }
    }
}
