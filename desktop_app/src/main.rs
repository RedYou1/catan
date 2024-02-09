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

#[macroquad::main(configure_window)]
async fn main() {
    let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
    let _puffin_server =
        puffin_http::Server::new(&server_addr).expect("can't open server for profiling");

    puffin::set_scopes_on(true);

    let mut state = Data::new();
    loop {
        clear_background(DARKGRAY);

        update_frame(&mut state).await;

        next_frame().await;
        profiling::finish_frame!();
    }
}

#[profiling::function]
async fn update_frame(state: &mut Data) {
    match state.page {
        Page::Main => game::game(state),
        Page::Reduce => reduce(state),
    }
}
