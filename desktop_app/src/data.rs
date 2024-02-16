use catan_lib::{game_manager::Game, ressource_manager::RessourceManager};
use macroquad::prelude::*;
use macroquadstate::{
    center::Center,
    range::Range,
    state::{DrawableState, State},
    v_stack::VStack,
};

use crate::{
    page::{game, reduce::reduce},
    player::Player,
    starting::Starting,
    Page,
};

#[derive(PartialEq)]
pub enum Thief {
    None,
    Waiting,
    Choosing,
}

#[allow(clippy::module_name_repetitions)]
pub type DataReturn = Center<VStack>;
#[deny(clippy::module_name_repetitions)]

pub struct Data {
    pub game: Game<Player, 4>,
    pub page: Page,
    pub to_reduce: RessourceManager,
    pub dices: Option<(u8, u8)>,
    pub debut: Starting,
    pub thief: Thief,
}

impl Data {
    pub fn new() -> Self {
        Self {
            game: Game::new(
                7,
                [
                    Player::new("Blue", BLUE),
                    Player::new("Red", RED),
                    Player::new("Green", GREEN),
                    Player::new("Yellow", YELLOW),
                ],
            )
            .expect("Couldn't create the game"),
            page: Page::Main,
            to_reduce: RessourceManager::default(),
            dices: None,
            debut: Starting::new(),
            thief: Thief::None,
        }
    }
}

#[profiling::all_functions]
impl DrawableState<DataReturn> for Data {
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

    fn gen_draw(state: &mut State<Data, DataReturn>) -> DataReturn {
        Center::new(match state.data().page {
            Page::Main => game::game(state),
            Page::Reduce => reduce(state),
        })
    }
}
