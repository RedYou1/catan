#![feature(const_trait_impl, effects)]

use data::{Data, DataReturn};
use macroquad::prelude::*;
use macroquadstate::button::Button;
use macroquadstate::drawable::Drawable;
use macroquadstate::range::Range;
use macroquadstate::state::{DrawableState, State, SubState};
use macroquadstate::v_stack::VStack;
use macroquadstate::vstack;
use macroquadstate::wrapper::Wrapper;
use std::thread::sleep;
use std::time::Duration;

mod data;
mod draw;
mod page;
mod player;

pub const HEX_SIZE: f32 = 50.0;
const FPS_IN_MILLIS: u64 = 1000 / 20;

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
    Game(SubState<Data, DataReturn>),
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
    pub fn unwrap_game(&mut self) -> &mut SubState<Data, DataReturn> {
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
        match &state.as_ref().page {
            Page::Main(data) => Wrapper::new(page::main::main(data.player_number, state)),
            Page::Game(gamedata) => {
                let content = gamedata.draw();
                Wrapper::new(vstack![
                    Button::new_stop("Retour", state, |data| {
                        data.page = Page::Main(MainData {
                            player_number: match &data.page {
                                Page::Game(gamedata) => gamedata.as_ref().data().game.players_len(),
                                Page::Main(data) => data.player_number,
                            },
                        });
                    }),
                    content,
                ])
            }
        }
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
        let start = std::time::Instant::now();
        clear_background(DARKGRAY);

        let _ = state.draw(0.0, 0.0, screen_width(), screen_height());

        #[allow(clippy::cast_possible_truncation)]
        let duration = start.elapsed().as_millis() as u64;
        if duration < FPS_IN_MILLIS {
            sleep(Duration::from_millis(FPS_IN_MILLIS - duration));
        }

        next_frame().await;
        profiling::finish_frame!();
    }
}
