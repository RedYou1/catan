#![feature(const_trait_impl, effects)]

use data::Data;
use macroquad::prelude::*;
use macroquadstate::drawable::Drawable;
use macroquadstate::state::State;

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

    let mut state = State::new(Data::new());
    loop {
        clear_background(DARKGRAY);

        state.draw(0.0, 0.0, screen_width(), screen_height());

        next_frame().await;
        profiling::finish_frame!();
    }
}
