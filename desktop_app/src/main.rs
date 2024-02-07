#![feature(const_trait_impl, effects)]

use catan_lib::game_manager::Game;
use catan_lib::ressource_manager::RessourceManager;
use macroquad::prelude::*;
use page::game;
use page::reduce::reduce;
use player::Player;

mod draw;
mod page;
mod player;

pub const HEX_SIZE: f32 = 50.0;

fn configure_window() -> Conf {
    Conf {
        window_title: String::from("Catan"),
        window_resizable: true,
        ..Default::default()
    }
}

pub enum Page {
    Main,
    Reduce,
}

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_lines)]
#[macroquad::main(configure_window)]
async fn main() {
    #[deny(clippy::needless_pass_by_value)]
    let mut game = Game::new(
        7,
        [
            Player::new("Blue", BLUE),
            Player::new("Red", RED),
            Player::new("Green", GREEN),
            Player::new("Yellow", YELLOW),
        ],
    )
    .expect("Couldn't create the game");
    let mut page = Page::Main;
    let mut to_reduce = RessourceManager::default();
    let mut dices: Option<(u8, u8)> = None;
    let mut debut = Starting::new();
    loop {
        clear_background(DARKGRAY);

        match page {
            Page::Main => game::game(&mut game, &mut page, &mut to_reduce, &mut dices, &mut debut),
            Page::Reduce => reduce(&mut game, &mut page, &mut to_reduce),
        }

        next_frame().await;
    }
}

struct Starting {
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
