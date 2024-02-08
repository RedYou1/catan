use crate::{
    building::Building, player::TPlayer, port::Port, position::Pos, ressource::Ressource,
    tile::Tile,
};

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
pub struct Game<Player: TPlayer, const PLAYERS_COUNT: usize> {
    max_ressource: u8,
    players: [Player; PLAYERS_COUNT],
    map: [[Option<Tile>; 5]; 5],
    ports: [Port; 9],
    building: [[Option<(Building, usize)>; 11]; 6],
    vroad: [[Option<usize>; 6]; 5],
    hroad: [[Option<usize>; 10]; 6],
    to_play: usize,
    thief: (usize, usize),
}

impl<Player: TPlayer + Default + Copy, const PLAYERS_COUNT: usize> Default
    for Game<Player, PLAYERS_COUNT>
{
    fn default() -> Self {
        Self {
            max_ressource: 0,
            players: [Player::default(); PLAYERS_COUNT],
            map: [[None; 5]; 5],
            ports: [Port::default(); 9],
            building: [[None; 11]; 6],
            vroad: [[None; 6]; 5],
            hroad: [[None; 10]; 6],
            to_play: 0,
            thief: (0, 0),
        }
    }
}

impl<Player: TPlayer, const PLAYERS_COUNT: usize> Game<Player, PLAYERS_COUNT> {
    pub fn new(max_ressource: u8, players: [Player; PLAYERS_COUNT]) -> Option<Self> {
        let mut dices = [2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
        let mut rng = rand::thread_rng();
        dices.shuffle(&mut rng);

        let mut tiles = [
            Some(Tile::new(Ressource::Tree, dices[0])),
            Some(Tile::new(Ressource::Tree, dices[1])),
            Some(Tile::new(Ressource::Tree, dices[2])),
            Some(Tile::new(Ressource::Tree, dices[3])),
            Some(Tile::new(Ressource::Wheet, dices[4])),
            Some(Tile::new(Ressource::Wheet, dices[5])),
            Some(Tile::new(Ressource::Wheet, dices[6])),
            Some(Tile::new(Ressource::Wheet, dices[7])),
            Some(Tile::new(Ressource::Brick, dices[8])),
            Some(Tile::new(Ressource::Brick, dices[9])),
            Some(Tile::new(Ressource::Brick, dices[10])),
            Some(Tile::new(Ressource::Sheep, dices[11])),
            Some(Tile::new(Ressource::Sheep, dices[12])),
            Some(Tile::new(Ressource::Sheep, dices[13])),
            Some(Tile::new(Ressource::Sheep, dices[14])),
            Some(Tile::new(Ressource::Stone, dices[15])),
            Some(Tile::new(Ressource::Stone, dices[16])),
            Some(Tile::new(Ressource::Stone, dices[17])),
            None,
        ];
        tiles.shuffle(&mut rng);
        let mut dessert_id = 0;
        for (i, item) in tiles.iter().enumerate() {
            if let None = item {
                dessert_id = i;
                break;
            }
        }

        let mut ports = [
            Some(Ressource::Tree),
            Some(Ressource::Wheet),
            Some(Ressource::Brick),
            Some(Ressource::Sheep),
            Some(Ressource::Stone),
            None,
            None,
            None,
            None,
        ];
        ports.shuffle(&mut rng);

        Some(Self {
            max_ressource,
            players,
            map: [
                [None, tiles[0], tiles[1], tiles[2], None],
                [tiles[3], tiles[4], tiles[5], tiles[6], None],
                [tiles[7], tiles[8], tiles[9], tiles[10], tiles[11]],
                [tiles[12], tiles[13], tiles[14], tiles[15], None],
                [None, tiles[16], tiles[17], tiles[18], None],
            ],
            ports: [
                Port::new(ports[0], Pos::new(2, 0), Pos::new(3, 0)),
                Port::new(ports[1], Pos::new(5, 0), Pos::new(6, 0)),
                Port::new(ports[2], Pos::new(8, 1), Pos::new(9, 1)),
                Port::new(ports[3], Pos::new(0, 1), Pos::new(0, 2)),
                Port::new(ports[4], Pos::new(10, 2), Pos::new(10, 3)),
                Port::new(ports[5], Pos::new(1, 3), Pos::new(1, 4)),
                Port::new(ports[6], Pos::new(9, 4), Pos::new(8, 4)),
                Port::new(ports[7], Pos::new(2, 5), Pos::new(3, 5)),
                Port::new(ports[8], Pos::new(5, 5), Pos::new(6, 5)),
            ],
            building: [[None; 11]; 6],
            vroad: [[None; 6]; 5],
            hroad: [[None; 10]; 6],
            to_play: rng.gen_range(0..PLAYERS_COUNT),
            thief: match dessert_id {
                0 => (1, 0),
                1 => (2, 0),
                2 => (3, 0),
                3 => (0, 1),
                4 => (1, 1),
                5 => (2, 1),
                6 => (3, 1),
                7 => (0, 2),
                8 => (1, 2),
                9 => (2, 2),
                10 => (3, 2),
                11 => (4, 2),
                12 => (0, 3),
                13 => (1, 3),
                14 => (2, 3),
                15 => (3, 3),
                16 => (1, 4),
                17 => (2, 4),
                18 => (3, 4),
                _ => (0, 0),
            },
        })
    }

    pub fn throw_dice(&mut self) -> (u8, u8) {
        let mut rng = rand::thread_rng();
        let dice_1: u8 = rng.gen_range(1..=6);
        let dice_2: u8 = rng.gen_range(1..=6);
        let dice = dice_1 + dice_2;
        if dice == 7 {
            return (dice_1, dice_2);
        }

        let mut amount: u8 = if dice == 2 || dice == 12 { 1 } else { 2 };

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let Some(tile) = tile else {
                    continue;
                };

                if tile.dice_id() != dice {
                    continue;
                }

                for by in y..=y + 1 {
                    for bx in x * 2..=(x + 1) * 2 {
                        let Some(building) = self.building[by][bx + (y % 2)] else {
                            continue;
                        };

                        self.players[building.1].ressources_mut().add(
                            tile.ressource(),
                            match building.0 {
                                Building::LittleHouse => 1,
                                Building::BigHouse => 2,
                            },
                        );
                    }
                }

                amount -= 1;
                if amount == 0 {
                    return (dice_1, dice_2);
                }
            }
        }
        (dice_1, dice_2)
    }

    pub const fn max_ressource(&self) -> u8 {
        self.max_ressource
    }

    pub const fn current_player_id(&self) -> usize {
        self.to_play
    }

    pub const fn current_player(&self) -> &Player {
        &self.players[self.to_play]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.to_play]
    }

    pub fn next_player(&mut self) {
        self.to_play += 1;
        if self.players.len() == self.to_play {
            self.to_play = 0;
        }
    }
    pub fn prev_player(&mut self) {
        if self.to_play == 0 {
            self.to_play = PLAYERS_COUNT - 1;
        } else {
            self.to_play -= 1;
        }
    }

    pub const fn player(&self, id: usize) -> &Player {
        &self.players[id]
    }

    pub fn players(&self) -> &[Player] {
        self.players.as_ref()
    }

    pub fn players_mut(&mut self) -> &mut [Player] {
        self.players.as_mut()
    }

    pub const fn building(&self, x: usize, y: usize) -> Option<&(Building, usize)> {
        self.building[y][x].as_ref()
    }

    pub fn building_mut(&mut self, x: usize, y: usize) -> &mut Option<(Building, usize)> {
        &mut self.building[y][x]
    }

    pub const fn vroad(&self, x: usize, y: usize) -> Option<&usize> {
        self.vroad[y][x].as_ref()
    }

    pub fn vroad_mut(&mut self, x: usize, y: usize) -> &mut Option<usize> {
        &mut self.vroad[y][x]
    }

    pub const fn hroad(&self, x: usize, y: usize) -> Option<&usize> {
        self.hroad[y][x].as_ref()
    }

    pub fn hroad_mut(&mut self, x: usize, y: usize) -> &mut Option<usize> {
        &mut self.hroad[y][x]
    }

    pub const fn tiles(&self) -> &[[Option<Tile>; 5]; 5] {
        &self.map
    }

    pub const fn thief(&self) -> &(usize, usize) {
        &self.thief
    }

    pub fn thief_mut(&mut self) -> &mut (usize, usize) {
        &mut self.thief
    }

    pub fn update_longuest_road(
        &mut self,
        x: usize,
        y: usize,
        is_vertical: bool,
        player_id: usize,
    ) {
        //TODO enemy's houses can broke the road?
        let mut score = 0;
        let mut remain_hroad = Vec::with_capacity(4);
        let mut remain_vroad = Vec::with_capacity(4);

        if is_vertical {
            remain_vroad.push((x, y));
        } else {
            remain_hroad.push((x, y));
        }

        loop {
            if !remain_hroad.is_empty() {
                let (x, y) = remain_hroad
                    .pop()
                    .expect("Empty array in update_longuest_road");
                remain_hroad.extend(
                    hroad_near_hroad(x, y)
                        .iter()
                        .filter(|a| self.hroad[a.1][a.0].map_or(false, |p| p == player_id)),
                );
                remain_vroad.extend(
                    vroad_near_hroad(x, y)
                        .iter()
                        .filter(|a| self.vroad[a.1][a.0].map_or(false, |p| p == player_id)),
                );
            } else if !remain_vroad.is_empty() {
                let (x, y) = remain_vroad
                    .pop()
                    .expect("Empty array in update_longuest_road");
                remain_hroad.extend(
                    hroad_near_vroad(x, y)
                        .iter()
                        .filter(|a| self.hroad[a.1][a.0].map_or(false, |p| p == player_id)),
                );
            } else {
                break;
            }
            score += 1;
        }

        let longuest_road = self.players[player_id].longuest_road_mut();
        if score > *longuest_road {
            *longuest_road = score;
        }
    }

    pub fn building_in_range(&self, x: usize, y: usize) -> bool {
        building_near_building(x, y)
            .iter()
            .any(|(x, y)| self.building[*y][*x].is_some())
    }
}

pub fn building_near_building(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut buildings: Vec<(usize, usize)> = hroad_near_building(x, y)
        .iter()
        .flat_map(|(x, y)| building_near_hroad(*x, *y))
        .collect();
    if let Some((x, y)) = vroad_near_building(x, y) {
        buildings.extend(building_near_vroad(x, y));
    }
    buildings
}
pub fn hroad_near_vroad(x: usize, y: usize) -> Vec<(usize, usize)> {
    let buildings = building_near_vroad(x, y);
    let mut hroads_1 = hroad_near_building(buildings[0].0, buildings[0].1);
    hroads_1.extend(hroad_near_building(buildings[1].0, buildings[1].1));
    hroads_1
}
pub fn vroad_near_hroad(x: usize, y: usize) -> Vec<(usize, usize)> {
    let buildings = building_near_hroad(x, y);
    let mut res = Vec::with_capacity(2);
    if let Some(vroad) = vroad_near_building(buildings[0].0, buildings[0].1) {
        res.push(vroad);
    }
    if let Some(vroad) = vroad_near_building(buildings[1].0, buildings[1].1) {
        res.push(vroad);
    }
    res
}
pub fn hroad_near_hroad(x: usize, y: usize) -> Vec<(usize, usize)> {
    if x == 0 {
        vec![(1, y)]
    } else if x == 9 {
        vec![(8, y)]
    } else {
        vec![(x - 1, y), (x + 1, y)]
    }
}
pub fn building_near_vroad(x: usize, y: usize) -> [(usize, usize); 2] {
    let off = y % 2;
    [(x * 2 + off, y), (x * 2 + off, y + 1)]
}
pub fn vroad_near_building(x: usize, y: usize) -> Option<(usize, usize)> {
    let off = if x % 2 == y % 2 { 0 } else { 1 };
    if y == 0 && off == 1 {
        return None;
    }
    if y == 5 && off == 0 {
        return None;
    }
    Some((x / 2, y - off))
}
pub fn building_near_hroad(x: usize, y: usize) -> [(usize, usize); 2] {
    [(x, y), (x + 1, y)]
}
pub fn hroad_near_building(x: usize, y: usize) -> Vec<(usize, usize)> {
    if x == 0 {
        vec![(0, y)]
    } else if x == 10 {
        vec![(9, y)]
    } else {
        vec![(x - 1, y), (x, y)]
    }
}
