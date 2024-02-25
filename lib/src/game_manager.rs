use crate::{
    building::Building, player::TPlayer, port::Port, position::Pos, ressource::Ressource,
    tile::Tile,
};

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
pub struct Game<Player: TPlayer> {
    max_ressource: u8,
    players: Vec<Player>,
    map: [[Option<Tile>; 5]; 5],
    ports: [Port; 9],
    building: [[Option<(Building, u8)>; 11]; 6],
    vroad: [[Option<u8>; 6]; 5],
    hroad: [[Option<u8>; 10]; 6],
    to_play: u8,
    thief: (u8, u8),
}

impl<Player: TPlayer> Game<Player> {
    #[allow(clippy::missing_panics_doc)]
    pub fn new(max_ressource: u8, players: Vec<Player>) -> Option<Self> {
        let plen = players.len();
        assert!(plen < 9);
        assert!(plen > 1);
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
            if item.is_none() {
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
                Port::new(ports[3], Pos::new(1, 1), Pos::new(1, 2)),
                Port::new(ports[4], Pos::new(10, 2), Pos::new(10, 3)),
                Port::new(ports[5], Pos::new(1, 3), Pos::new(1, 4)),
                Port::new(ports[6], Pos::new(9, 4), Pos::new(8, 4)),
                Port::new(ports[7], Pos::new(2, 5), Pos::new(3, 5)),
                Port::new(ports[8], Pos::new(5, 5), Pos::new(6, 5)),
            ],
            building: [[None; 11]; 6],
            vroad: [[None; 6]; 5],
            hroad: [[None; 10]; 6],
            to_play: u8::try_from(rng.gen_range(0..plen)).expect("random out of bound"),
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

    #[allow(clippy::missing_panics_doc)]
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

                for (bx, by) in
                    building_around_tile(u8::try_from(x).expect(""), u8::try_from(y).expect(""))
                {
                    let Some(building) = self.building[by as usize][bx as usize] else {
                        continue;
                    };

                    self.players[building.1 as usize].ressources_mut().add(
                        tile.ressource(),
                        match building.0 {
                            Building::LittleHouse => 1,
                            Building::BigHouse => 2,
                        },
                    );
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

    pub const fn current_player_id(&self) -> u8 {
        self.to_play
    }

    pub const fn current_player(&self) -> &Player {
        &self.players[self.to_play as usize]
    }

    pub fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.to_play as usize]
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

    pub const fn building(&self, x: u8, y: u8) -> Option<&(Building, u8)> {
        self.building[y as usize][x as usize].as_ref()
    }

    pub fn building_mut(&mut self, x: u8, y: u8) -> &mut Option<(Building, u8)> {
        &mut self.building[y as usize][x as usize]
    }

    pub const fn vroad(&self, x: u8, y: u8) -> Option<&u8> {
        self.vroad[y as usize][x as usize].as_ref()
    }

    pub fn vroad_mut(&mut self, x: u8, y: u8) -> &mut Option<u8> {
        &mut self.vroad[y as usize][x as usize]
    }

    pub const fn hroad(&self, x: u8, y: u8) -> Option<&u8> {
        self.hroad[y as usize][x as usize].as_ref()
    }

    pub fn hroad_mut(&mut self, x: u8, y: u8) -> &mut Option<u8> {
        &mut self.hroad[y as usize][x as usize]
    }

    pub const fn tiles(&self) -> &[[Option<Tile>; 5]; 5] {
        &self.map
    }

    pub const fn thief(&self) -> &(u8, u8) {
        &self.thief
    }

    pub fn thief_mut(&mut self) -> &mut (u8, u8) {
        &mut self.thief
    }

    pub const fn ports(&self) -> &[Port; 9] {
        &self.ports
    }

    #[allow(clippy::similar_names)]
    #[allow(clippy::missing_panics_doc)]
    pub fn update_longuest_road(&mut self, x: u8, y: u8, is_vertical: bool, player_id: u8) {
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
                remain_hroad.extend(hroad_near_hroad(x, y).iter().filter(|a| {
                    self.hroad[a.1 as usize][a.0 as usize].map_or(false, |p| p == player_id)
                }));
                remain_vroad.extend(vroad_near_hroad(x, y).iter().filter(|a| {
                    self.vroad[a.1 as usize][a.0 as usize].map_or(false, |p| p == player_id)
                }));
            } else if !remain_vroad.is_empty() {
                let (x, y) = remain_vroad
                    .pop()
                    .expect("Empty array in update_longuest_road");
                remain_hroad.extend(hroad_near_vroad(x, y).iter().filter(|a| {
                    self.hroad[a.1 as usize][a.0 as usize].map_or(false, |p| p == player_id)
                }));
            } else {
                break;
            }
            score += 1;
        }

        let longuest_road = self.players[player_id as usize].longuest_road_mut();
        if score > *longuest_road {
            *longuest_road = score;
        }
    }

    pub fn building_in_range(&self, x: u8, y: u8) -> bool {
        building_near_building(x, y)
            .iter()
            .any(|(x, y)| self.building[*y as usize][*x as usize].is_some())
    }

    pub fn steal(&mut self, from: u8, to: u8) {
        let mut rng = rand::thread_rng();
        let from = self.players[from as usize].ressources_mut();
        let ressources: Vec<Ressource> = from
            .gets()
            .iter()
            .filter(|(_, amount)| *amount > 0)
            .map(|(ressource, _)| *ressource)
            .collect();
        if ressources.is_empty() {
            return;
        }
        let ressource = ressources[rng.gen_range(0..ressources.len())];
        from.sub(ressource, 1);
        let to = self.players[to as usize].ressources_mut();
        to.add(ressource, 1);
    }
}

pub const fn building_around_tile(x: u8, y: u8) -> [(u8, u8); 6] {
    let x = x * 2 + (y % 2);
    [
        (x, y),
        (x + 1, y),
        (x + 2, y),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 2, y + 1),
    ]
}
pub fn building_near_building(x: u8, y: u8) -> Vec<(u8, u8)> {
    let mut buildings: Vec<(u8, u8)> = hroad_near_building(x, y)
        .iter()
        .flat_map(|(x, y)| building_near_hroad(*x, *y))
        .collect();
    if let Some((x, y)) = vroad_near_building(x, y) {
        buildings.extend(building_near_vroad(x, y));
    }
    buildings
}
pub fn hroad_near_vroad(x: u8, y: u8) -> Vec<(u8, u8)> {
    let buildings = building_near_vroad(x, y);
    let mut hroads_1 = hroad_near_building(buildings[0].0, buildings[0].1);
    hroads_1.extend(hroad_near_building(buildings[1].0, buildings[1].1));
    hroads_1
}
pub fn vroad_near_hroad(x: u8, y: u8) -> Vec<(u8, u8)> {
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
pub fn hroad_near_hroad(x: u8, y: u8) -> Vec<(u8, u8)> {
    if x == 0 {
        vec![(1, y)]
    } else if x == 9 {
        vec![(8, y)]
    } else {
        vec![(x - 1, y), (x + 1, y)]
    }
}
pub const fn building_near_vroad(x: u8, y: u8) -> [(u8, u8); 2] {
    let off = y % 2;
    [(x * 2 + off, y), (x * 2 + off, y + 1)]
}
pub const fn vroad_near_building(x: u8, y: u8) -> Option<(u8, u8)> {
    let off = if x % 2 == y % 2 { 0 } else { 1 };
    if y == 0 && off == 1 {
        return None;
    }
    if y == 5 && off == 0 {
        return None;
    }
    Some((x / 2, y - off))
}
pub const fn building_near_hroad(x: u8, y: u8) -> [(u8, u8); 2] {
    [(x, y), (x + 1, y)]
}
pub fn hroad_near_building(x: u8, y: u8) -> Vec<(u8, u8)> {
    if x == 0 {
        vec![(0, y)]
    } else if x == 10 {
        vec![(9, y)]
    } else {
        vec![(x - 1, y), (x, y)]
    }
}
