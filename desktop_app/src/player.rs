use catan_lib::player::TPlayer;
use catan_lib::ressource_manager::RessourceManager;
use macroquad::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Player {
    name: String,
    color: Color,
    road_remaining: u8,
    house_remaining: u8,
    big_house_remaining: u8,
    longuest_road: u8,
    army: u8,
    ressources: RessourceManager,
}

impl TPlayer for Player {
    fn road_remaining(&self) -> u8 {
        self.road_remaining
    }
    fn road_remaining_mut(&mut self) -> &mut u8 {
        &mut self.road_remaining
    }

    fn house_remaining(&self) -> u8 {
        self.house_remaining
    }
    fn house_remaining_mut(&mut self) -> &mut u8 {
        &mut self.house_remaining
    }

    fn big_house_remaining(&self) -> u8 {
        self.big_house_remaining
    }
    fn big_house_remaining_mut(&mut self) -> &mut u8 {
        &mut self.big_house_remaining
    }

    fn longuest_road(&self) -> u8 {
        self.longuest_road
    }
    fn longuest_road_mut(&mut self) -> &mut u8 {
        &mut self.longuest_road
    }

    fn army(&self) -> u8 {
        self.army
    }
    fn army_mut(&mut self) -> &mut u8 {
        &mut self.army
    }

    fn ressources(&self) -> &RessourceManager {
        &self.ressources
    }

    fn ressources_mut(&mut self) -> &mut RessourceManager {
        &mut self.ressources
    }
}

impl Player {
    pub const fn new(name: String, color: Color) -> Self {
        Self {
            name,
            color,
            road_remaining: 15,
            house_remaining: 5,
            big_house_remaining: 4,
            longuest_road: 0,
            army: 0,
            ressources: RessourceManager::default(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub const fn color(&self) -> Color {
        self.color
    }
}
