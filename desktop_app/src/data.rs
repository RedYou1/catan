use catan_lib::{game_manager::Game, ressource_manager::RessourceManager};
use macroquad::prelude::*;

use crate::{player::Player, starting::Starting, Page};

#[derive(PartialEq)]
pub enum Thief {
    None,
    Waiting,
    Choosing,
}

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
