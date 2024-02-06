use crate::ressource::Ressource;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RessourceManager {
    trees: u8,
    wheets: u8,
    bricks: u8,
    sheeps: u8,
    stone: u8,
}

impl RessourceManager {
    pub const fn amounts(self) -> u8 {
        self.trees + self.wheets + self.bricks + self.sheeps + self.stone
    }

    pub const fn get(self, ressource: Ressource) -> u8 {
        match ressource {
            Ressource::Tree => self.trees,
            Ressource::Wheet => self.wheets,
            Ressource::Brick => self.bricks,
            Ressource::Sheep => self.sheeps,
            Ressource::Stone => self.stone,
        }
    }

    pub const fn can_buy(self, trees: u8, wheets: u8, bricks: u8, sheeps: u8, stone: u8) -> bool {
        self.trees >= trees
            && self.wheets >= wheets
            && self.bricks >= bricks
            && self.sheeps >= sheeps
            && self.stone >= stone
    }

    pub fn add(&mut self, ressource: Ressource, amount: u8) {
        match ressource {
            Ressource::Tree => self.trees += amount,
            Ressource::Wheet => self.wheets += amount,
            Ressource::Brick => self.bricks += amount,
            Ressource::Sheep => self.sheeps += amount,
            Ressource::Stone => self.stone += amount,
        };
    }

    pub fn sub(&mut self, ressource: Ressource, amount: u8) {
        match ressource {
            Ressource::Tree => self.trees -= amount,
            Ressource::Wheet => self.wheets -= amount,
            Ressource::Brick => self.bricks -= amount,
            Ressource::Sheep => self.sheeps -= amount,
            Ressource::Stone => self.stone -= amount,
        };
    }

    pub fn subs(&mut self, ressources: RessourceManager) {
        self.trees -= ressources.trees;
        self.wheets -= ressources.wheets;
        self.bricks -= ressources.bricks;
        self.sheeps -= ressources.sheeps;
        self.stone -= ressources.stone;
    }

    pub fn buy(&mut self, trees: u8, wheets: u8, bricks: u8, sheeps: u8, stone: u8) {
        self.trees -= trees;
        self.wheets -= wheets;
        self.bricks -= bricks;
        self.sheeps -= sheeps;
        self.stone -= stone;
    }
}
