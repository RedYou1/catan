use crate::{
    building::Building,
    game_coords::{building_around_tile, hroad_near_hroad, hroad_near_vroad, vroad_near_hroad},
    player::TPlayer,
    port::Port,
    position::Pos,
    ressource::Ressource,
    starting::Starting,
    tile::Tile,
};
use rand::seq::SliceRandom;
use rand::Rng;

pub mod buildings;
pub mod hroads;
pub mod players;
pub mod vroads;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Thief {
    None,
    Waiting,
    Choosing,
}

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
    thief_coords: (u8, u8),
    debut: Starting,
    thief_state: Thief,
}

impl<Player: TPlayer> Game<Player> {
    #[allow(clippy::missing_panics_doc)]
    pub fn new(max_ressource: u8, players: Vec<Player>) -> Option<Self> {
        let plen = players.len();
        assert!(plen < 9);
        assert!(plen > 1);
        #[allow(clippy::cast_possible_truncation)]
        let plen = plen as u8;
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
            to_play: u8::try_from(rng.gen_range(0..plen as usize)).expect("random out of bound"),
            thief_coords: match dessert_id {
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
                _ => panic!("tile id out of bound"),
            },
            debut: Starting::new(plen),
            thief_state: Thief::None,
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

    pub const fn tiles(&self) -> &[[Option<Tile>; 5]; 5] {
        &self.map
    }

    pub const fn thief_coords(&self) -> (u8, u8) {
        self.thief_coords
    }

    pub fn set_thief_coords(&mut self, x: u8, y: u8) {
        self.thief_coords = (x, y);
    }

    pub const fn thief_state(&self) -> Thief {
        self.thief_state
    }

    pub fn set_thief_state(&mut self, thief: Thief) {
        self.thief_state = thief;
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

    pub const fn is_starting(&self) -> bool {
        self.debut.is_starting()
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
