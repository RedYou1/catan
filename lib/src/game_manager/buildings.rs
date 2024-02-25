use crate::{
    building::Building,
    game_coords::{building_near_building, hroad_near_building, vroad_near_building},
    game_manager::Game,
    player::TPlayer,
};

impl<Player: TPlayer> Game<Player> {
    pub const fn building(&self, x: u8, y: u8) -> Option<&(Building, u8)> {
        self.building[y as usize][x as usize].as_ref()
    }

    pub fn building_in_range(&self, x: u8, y: u8) -> bool {
        building_near_building(x, y)
            .iter()
            .any(|(x, y)| self.building[*y as usize][*x as usize].is_some())
    }

    pub fn can_place_building(&self, x: u8, y: u8, current_playing: u8) -> bool {
        (self.debut.building_turn() || self.current_player().ressources().can_buy(1, 1, 1, 1, 0))
            && !self.building_in_range(x, y)
            && (self.debut.building_turn()
                || hroad_near_building(x, y)
                    .into_iter()
                    .any(|(x1, y1)| self.hroad(x1, y1).map_or(false, |&a| a == current_playing))
                || vroad_near_building(x, y)
                    .into_iter()
                    .any(|(x1, y1)| self.vroad(x1, y1).map_or(false, |&a| a == current_playing)))
            && !self.building_in_range(x, y)
    }

    pub fn buy_building(&mut self, x: u8, y: u8) {
        self.building[y as usize][x as usize] =
            Some((Building::LittleHouse, self.current_player_id()));
        if self.debut.building_turn() {
            self.debut.place_building(x, y);
        } else {
            self.players[self.to_play as usize]
                .ressources_mut()
                .buy(1, 1, 1, 1, 0);
        }
    }

    pub fn can_upgrade_building(&self, player_id: u8) -> bool {
        self.players[player_id as usize]
            .ressources()
            .can_buy(0, 2, 0, 0, 3)
    }

    pub fn upgrade_building(&mut self, x: u8, y: u8, player_id: u8) {
        self.building[y as usize][x as usize] = Some((Building::BigHouse, player_id));
        self.players[self.to_play as usize]
            .ressources_mut()
            .buy(0, 2, 0, 0, 3);
    }
}
