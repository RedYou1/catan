#![feature(const_trait_impl, effects)]

use catan_lib::game_manager::Game;
use macroquad::prelude::*;
use page::game;
use page::reduce::reduce;
use player::Player;
use state::State;

mod draw;
mod page;
mod player;
mod starting;
mod state;

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
    let mut state = State::new();
    loop {
        clear_background(DARKGRAY);

        match state.page {
            Page::Main => game::game(&mut game, &mut state),
            Page::Reduce => reduce(&mut game, &mut state),
        }

        next_frame().await;
    }
}
