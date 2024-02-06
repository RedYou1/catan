use catan_lib::player::TPlayer;
use catan_lib::ressource_manager::RessourceManager;
use macroquad::color::Color;

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    name: &'static str,
    color: Color,
    ressources: RessourceManager,
}

impl TPlayer for Player {
    fn ressources(&self) -> &RessourceManager {
        &self.ressources
    }

    fn ressources_mut(&mut self) -> &mut RessourceManager {
        &mut self.ressources
    }
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
}
