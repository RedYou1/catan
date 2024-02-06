use crate::{
    building::Building, player::TPlayer, port::Port, position::Pos, ressource::Ressource, tile::Tile,
};

use rand::rngs::mock::StepRng;
use rand::Rng;
use shuffle::irs::Irs;
use shuffle::shuffler::Shuffler;

#[derive(Debug)]
pub struct Game<Player: TPlayer, const PLAYERS_COUNT: usize> {
    max_ressource: u8,
    players: [Player; PLAYERS_COUNT],
    map: [[Option<Tile>; 5]; 5],
    ports: [Port; 9],
    building: [[Option<(Building, usize)>; 11]; 6],
    to_play: usize,
}

impl<Player: TPlayer + Default + Copy, const PLAYERS_COUNT: usize> Default for Game<Player, PLAYERS_COUNT> {
    fn default() -> Self {
        Self {
            max_ressource: 0,
            players: [Player::default(); PLAYERS_COUNT],
            map: [[None; 5]; 5],
            ports: [Port::default(); 9],
            building: [[None; 11]; 6],
            to_play: 0,
        }
    }
}

impl<Player: TPlayer, const PLAYERS_COUNT: usize> Game<Player, PLAYERS_COUNT> {
    pub fn new(
        max_ressource: u8,
        players: [Player; PLAYERS_COUNT],
    ) -> Option<Self> {
        let mut dices = vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
        let mut rng = StepRng::new(2, 13);
        Irs::default().shuffle(&mut dices, &mut rng).ok()?;

        let mut tiles = vec![
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
        let mut rng = StepRng::new(2, 13);
        Irs::default().shuffle(&mut tiles, &mut rng).ok()?;

        let mut ports = vec![
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
        let mut rng = StepRng::new(2, 13);
        Irs::default().shuffle(&mut ports, &mut rng).ok()?;

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
            to_play: 0,
        })
    }

    pub fn throw_dice(&mut self) -> (u8, u8) {
        let dice_1: u8 = rand::thread_rng().gen_range(1..=6);
        let dice_2: u8 = rand::thread_rng().gen_range(1..=6);
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

    pub const fn tiles(&self) -> &[[Option<Tile>; 5]; 5] {
        &self.map
    }
}
