#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ressource {
    #[default]
    Tree,
    Wheet,
    Brick,
    Sheep,
    Stone,
}

impl Ressource {
    pub const fn name(self) -> &'static str {
        match self {
            Ressource::Tree => "Tree",
            Ressource::Wheet => "Wheet",
            Ressource::Brick => "Brick",
            Ressource::Sheep => "Sheep",
            Ressource::Stone => "Stone",
        }
    }
}
