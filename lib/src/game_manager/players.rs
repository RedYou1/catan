use crate::{game_manager::Game, player::TPlayer};

impl<Player: TPlayer> Game<Player> {
    pub const fn player(&self, id: u8) -> &Player {
        &self.players[id as usize]
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn players_len(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn players(&self) -> &[Player] {
        self.players.as_ref()
    }

    pub fn players_mut(&mut self) -> &mut [Player] {
        self.players.as_mut()
    }

    pub const fn current_player_id(&self) -> u8 {
        self.to_play
    }

    pub const fn current_player(&self) -> &Player {
        &self.players[self.to_play as usize]
    }

    pub fn next_player(&mut self) {
        self.to_play += 1;
        if self.players.len() == self.to_play as usize {
            self.to_play = 0;
        }
    }
    #[allow(clippy::missing_panics_doc)]
    pub fn prev_player(&mut self) {
        if self.to_play == 0 {
            self.to_play = u8::try_from(self.players.len() - 1).expect("");
        } else {
            self.to_play -= 1;
        }
    }
}
