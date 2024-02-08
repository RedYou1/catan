use catan_lib::ressource_manager::RessourceManager;

use crate::{starting::Starting, Page};

#[derive(PartialEq)]
pub enum Thief {
    No,
    Waiting,
    Done,
}

pub struct State {
    pub page: Page,
    pub to_reduce: RessourceManager,
    pub dices: Option<(u8, u8)>,
    pub debut: Starting,
    pub thief: Thief,
}

impl State {
    pub fn new() -> Self {
        Self {
            page: Page::Main,
            to_reduce: RessourceManager::default(),
            dices: None,
            debut: Starting::new(),
            thief: Thief::No,
        }
    }
}
