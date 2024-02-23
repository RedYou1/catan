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

    pub const fn ressource(&self) -> Option<Ressource> {
        self.ressource
    }

    pub const fn pos1(&self) -> Pos {
        self.pos1
    }

    pub const fn pos2(&self) -> Pos {
        self.pos2
    }
}
