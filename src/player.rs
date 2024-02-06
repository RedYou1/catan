use macroquad::color::Color;

use crate::ressource_manager::RessourceManager;

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    name: &'static str,
    color: Color,
    ressources: RessourceManager,
}

impl Player {
    pub const fn new(name: &'static str, color: Color) -> Self {
        Self {
            name,
            color,
            ressources: RessourceManager::default(),
        }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn color(&self) -> Color {
        self.color
    }

    pub const fn ressources(&self) -> &RessourceManager {
        &self.ressources
    }

    pub fn ressources_mut(&mut self) -> &mut RessourceManager {
        &mut self.ressources
    }
}
