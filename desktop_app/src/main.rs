#![feature(const_trait_impl, effects)]

use data::{Data, DataReturn};
use macroquad::prelude::*;
use macroquadstate::button::Button;
use macroquadstate::drawable::Drawable;
use macroquadstate::range::Range;
use macroquadstate::state::{DrawableState, State};
use macroquadstate::v_stack::VStack;
use macroquadstate::vstack;
use macroquadstate::wrapper::{RefWrapper, Wrapper};

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

pub struct MainData {
    player_number: u8,
}

#[allow(clippy::large_enum_variant)]
pub enum Page {
    Main(MainData),
    Game(State<Data, DataReturn>),
}

#[allow(clippy::missing_panics_doc, clippy::match_wildcard_for_single_variants)]
impl Page {
    pub fn unwrap_main(&self) -> &MainData {
        match self {
            Page::Main(data) => data,
            _ => panic!("unwrap main"),
        }
    }
    pub fn unwrap_main_mut(&mut self) -> &mut MainData {
        match self {
            Page::Main(data) => data,
            _ => panic!("unwrap main"),
        }
    }
    pub fn unwrap_game(&mut self) -> &mut State<Data, DataReturn> {
        match self {
            Page::Game(data) => data,
            _ => panic!("unwrap game"),
        }
    }
}

pub type WindowReturn = Wrapper;

pub struct Window {
    page: Page,
}

#[profiling::all_functions]
impl DrawableState<WindowReturn> for Window {
    fn state_width(&self) -> Range {
        let w = screen_width();
        Range {
            min: w,
            max: Some(w),
        }
    }

    fn state_height(&self) -> Range {
        let h = screen_height();
        Range {
            min: h,
            max: Some(h),
        }
    }
    fn gen_draw(state: &mut State<Self, WindowReturn>) -> WindowReturn {
        state.draw_sub(|state, data| match &mut data.page {
            Page::Main(data) => {
                Wrapper::new(page::main::main(data, unsafe { state.as_mut().expect("") }))
            }
            Page::Game(gamedata) => Wrapper::new(vstack![
                Button::new_stop("Retour", state, |data| {
                    data.page = Page::Main(MainData {
                        player_number: match &data.page {
                            Page::Game(gamedata) => gamedata.data().game.players_len(),
                            Page::Main(data) => data.player_number,
                        },
                    });
                }),
                RefWrapper::new(gamedata),
            ]),
        })
    }
}

#[macroquad::main(configure_window)]
async fn main() {
    let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
    let _puffin_server =
        puffin_http::Server::new(&server_addr).expect("can't open server for profiling");

    puffin::set_scopes_on(true);

    let mut state = State::new(Window {
        page: Page::Main(MainData { player_number: 4 }),
    });
    loop {
        clear_background(DARKGRAY);

        let _ = state.draw(0.0, 0.0, screen_width(), screen_height());

        next_frame().await;
        profiling::finish_frame!();
    }
}
