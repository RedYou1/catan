use catan_lib::{game_manager::Game, ressource_manager::RessourceManager};
use macroquad::prelude::*;
use macroquadstate::{
    center::Center,
    empty::Empty,
    range::Range,
    state::{DrawableState, State},
    wrapper::Wrapper,
};

use crate::{
    page::{game, reduce::reduce},
    player::Player,
};

#[allow(clippy::module_name_repetitions)]
pub type DataReturn = Wrapper;
#[deny(clippy::module_name_repetitions)]

pub enum GamePage {
    Game,
    Reduce,
}

pub struct Data {
    pub game: Game<Player>,
    pub page: GamePage,
    pub to_reduce: RessourceManager,
    pub dices: Option<(u8, u8)>,
}

const PLAYER_COLOR: [Color; 8] = [BLUE, RED, GREEN, YELLOW, PURPLE, WHITE, BLACK, ORANGE];

impl Data {
    pub fn new(player_number: u8) -> Self {
        Self {
            game: Game::new(
                7,
                (1..=player_number)
                    .collect::<Vec<u8>>()
                    .iter()
                    .map(|i| Player::new(format!("P{i}"), PLAYER_COLOR[*i as usize - 1]))
                    .collect(),
            )
            .expect("Couldn't create the game"),
            page: GamePage::Game,
            to_reduce: RessourceManager::default(),
            dices: None,
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
        match state.data().page {
            GamePage::Game => Wrapper::new(Center::new(game::game(state))),
            GamePage::Reduce => {
                if let Some(reduce) = reduce(state) {
                    Wrapper::new(Center::new(reduce))
                } else {
                    Wrapper::new(Empty::new())
                }
            }
        }
    }
}
