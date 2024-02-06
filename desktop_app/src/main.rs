#![feature(const_trait_impl, effects)]

use catan_lib::game_manager::Game;
use catan_lib::player::TPlayer;
use catan_lib::ressource::Ressource;
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
    loop {
        clear_background(DARKGRAY);

        match page {
            Page::Main => game::game(&mut game, &mut page, &mut to_reduce, &mut dices),
            Page::Reduce => reduce(&mut game, &mut page, &mut to_reduce),
        }

        next_frame().await;
    }
}
