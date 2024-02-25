use crate::{
    game_coords::{building_near_hroad, hroad_near_hroad, vroad_near_hroad},
    game_manager::Game,
    player::TPlayer,
};

impl<Player: TPlayer> Game<Player> {
    pub const fn hroad(&self, x: u8, y: u8) -> Option<&u8> {
        self.hroad[y as usize][x as usize].as_ref()
    }

    pub fn can_place_hroad(&self, x: u8, y: u8) -> bool {
        (self.debut.road_turn() || self.current_player().ressources().can_buy(1, 0, 1, 0, 0))
            && if self.debut.road_turn() {
                building_near_hroad(x, y)
                    .iter()
                    .any(|(x1, y1)| self.debut.near_building(*x1, *y1))
            } else {
                hroad_near_hroad(x, y)
                    .iter()
                    .any(|(x1, y1)| self.hroad(*x1, *y1).map_or(false, |&a| a == self.to_play))
                    || vroad_near_hroad(x, y)
                        .iter()
                        .any(|(x1, y1)| self.vroad(*x1, *y1).map_or(false, |&a| a == self.to_play))
            }
    }

    pub fn buy_hroad(&mut self, x: u8, y: u8) {
        self.hroad[y as usize][x as usize] = Some(self.current_player_id());
        if self.debut.road_turn() {
            if let Some(dir) = self.debut.place_road(self.players_len()) {
                if dir {
                    self.prev_player();
                } else {
                    self.next_player();
                }
            }
        } else {
            self.players[self.to_play as usize]
                .ressources_mut()
                .buy(1, 0, 1, 0, 0);
        }
    }
}
