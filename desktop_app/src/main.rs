#![feature(const_trait_impl, effects)]

use data::Data;
use macroquad::prelude::*;
use page::game;
use page::reduce::reduce;

mod data;
mod draw;
mod page;
mod player;
mod starting;

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
    let mut state = Data::new();
    loop {
        clear_background(DARKGRAY);

        match state.page {
            Page::Main => game::game(&mut state),
            Page::Reduce => reduce(&mut state),
        }

        next_frame().await;
    }
}
