use crate::ressource::Ressource;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    ressource: Ressource,
    dice_id: u8,
}

impl Tile {
    pub const fn new(ressource: Ressource, dice_id: u8) -> Self {
        Self { ressource, dice_id }
    }

    pub const fn ressource(self) -> Ressource {
        self.ressource
    }

    pub const fn dice_id(self) -> u8 {
        self.dice_id
    }
}
