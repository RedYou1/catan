use crate::{position::Pos, ressource::Ressource};

#[derive(Debug, Default, Clone, Copy)]
pub struct Port {
    ressource: Option<Ressource>,
    pos1: Pos,
    pos2: Pos,
}

impl Port {
    pub const fn new(ressource: Option<Ressource>, pos1: Pos, pos2: Pos) -> Self {
        Self {
            ressource,
            pos1,
            pos2,
        }
    }
}
