use catan_lib::game_manager::Game;

use crate::player::Player;

pub struct Starting {
    building: bool,
    step: usize,
    building_x: usize,
    building_y: usize,
}

impl Starting {
    pub const fn new() -> Self {
        Self {
            building: false,
            step: 4 * 2,
            building_x: 0,
            building_y: 0,
        }
    }
    pub fn place_building(&mut self, x: usize, y: usize) {
        self.building_x = x;
        self.building_y = y;
        self.building = true;
    }
    pub fn place_road(&mut self, game: &mut Game<Player, 4>) {
        self.building = false;
        self.step -= 1;
        if self.step == 4 || self.step == 0 {
        } else if self.step < 4 {
            game.prev_player();
        } else {
            game.next_player();
        }
    }
    pub const fn near_building(&self, x: usize, y: usize) -> bool {
        self.building_x == x && self.building_y == y
    }
    pub const fn is_starting(&self) -> bool {
        self.step > 0
    }
    pub const fn building_turn(&self) -> bool {
        self.step > 0 && !self.building
    }
    pub const fn road_turn(&self) -> bool {
        self.step > 0 && self.building
    }
}
