use catan_lib::player::TPlayer;
use catan_lib::ressource_manager::RessourceManager;
use macroquad::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct Player {
    name: &'static str,
    color: Color,
    road_remaining: usize,
    house_remaining: usize,
    big_house_remaining: usize,
    longuest_road: usize,
    military: usize,
    ressources: RessourceManager,
}

impl TPlayer for Player {
    fn road_remaining(&self) -> usize {
        self.road_remaining
    }
    fn road_remaining_mut(&mut self) -> &mut usize {
        &mut self.road_remaining
    }

    fn house_remaining(&self) -> usize {
        self.house_remaining
    }
    fn house_remaining_mut(&mut self) -> &mut usize {
        &mut self.house_remaining
    }

    fn big_house_remaining(&self) -> usize {
        self.big_house_remaining
    }
    fn big_house_remaining_mut(&mut self) -> &mut usize {
        &mut self.big_house_remaining
    }

    fn longuest_road(&self) -> usize {
        self.longuest_road
    }
    fn longuest_road_mut(&mut self) -> &mut usize {
        &mut self.longuest_road
    }

    fn military(&self) -> usize {
        self.military
    }
    fn military_mut(&mut self) -> &mut usize {
        &mut self.military
    }

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
            road_remaining: 15,
            house_remaining: 5,
            big_house_remaining: 4,
            longuest_road: 0,
            military: 0,
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
