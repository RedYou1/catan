#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    pub const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub const fn x(&self) -> u8 {
        self.x
    }

    pub const fn y(&self) -> u8 {
        self.y
    }
}
