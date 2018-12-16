pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Dimension { width, height }
    }
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }
}
