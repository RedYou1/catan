use catan_lib::{
    game_coords::building_around_tile,
    game_manager::{Game, Thief},
    player::TPlayer,
    ressource_manager::RessourceManager,
};
use macroquad::prelude::*;
use macroquadstate::{
    button::Button,
    center::{CenterH, CenterV},
    drawable::Drawable,
    empty::Empty,
    fix_text::FixText,
    margin::Margin,
    space::Space,
    state::State,
    v_stack::VStack,
    vec_h_stack::VecHStack,
    vec_v_stack::VecVStack,
    vstack,
    z_stack::ZStack,
};

use crate::{
    data::{Data, DataReturn, GamePage},
    draw::{
        building::building,
        port,
        road::{hroad, vroad},
        tile,
    },
    player::Player,
};

#[profiling::function]
pub fn game(state: &mut State<Data, DataReturn>) -> VStack<4> {
    let data = state.data();
    let current_player = data.game.current_player();

    let to_choose: (u8, u8, u8) = {
        if data.game.is_starting() {
            (0, 0, 0)
        } else if let Some((a, b)) = data.dices {
            match data.game.thief_state() {
                Thief::Waiting => (0, a, b),
                Thief::Choosing => (1, a, b),
                Thief::None => (2, a, b),
            }
        } else {
            (3, 0, 0)
        }
    };
    VStack::new([
        Box::new(CenterH::new(Margin::news(
            FixText::new(
                format!("Player to play: {}", current_player.name()),
                25,
                current_player.color(),
            ),
            2.5,
        ))),
        Box::new(CenterH::new(VecVStack::new(
            data.game
                .players()
                .iter()
                .map(|player| {
                    Box::new(Margin::news(
                        FixText::new(
                            format!(
                                "Player:{} LR:{} Army:{} {}",
                                player.name(),
                                player.longuest_road(),
                                player.army(),
                                player.ressources().to_string()
                            ),
                            20,
                            player.color(),
                        ),
                        2.0,
                    )) as Box<dyn Drawable>
                })
                .collect(),
        ))),
        Box::new(CenterH::new(Margin::news(draw_map(state), 10.0))),
        match to_choose {
            (1, _, _) => {
                if let Some(steel) = choose_steal(state) {
                    Box::new(CenterH::new(steel))
                } else {
                    Box::new(Empty::new())
                }
            }
            (2, a, b) => Box::new(CenterH::new(vstack![
                CenterH::new(Margin::news(
                    FixText::new(format!("{a} {b}"), 25, WHITE),
                    5.0
                )),
                CenterH::new(Margin::news(
                    Button::new("Next", state, |data| {
                        data.game.next_player();
                        data.dices = None;
                    }),
                    5.0
                ))
            ])),
            (3, _, _) => Box::new(CenterH::new(Margin::news(
                Button::new("Dice", state, |data| {
                    let (a, b) = data.game.throw_dice();
                    data.dices = Some((a, b));
                    data.game.set_thief_state(if a + b == 7 {
                        Thief::Waiting
                    } else {
                        Thief::None
                    });
                    if a + b == 7 {
                        data.to_reduce = RessourceManager::default();
                        data.page = GamePage::Reduce;
                    }
                }),
                10.0,
            ))),
            _ => Box::new(Empty::new()),
        },
    ])
}

fn steel_posibility(player_id: u8, game: &Game<Player>) -> Vec<(u8, &str)> {
    let mut players: Vec<(u8, &str)> = Vec::with_capacity(game.players_len() as usize);
    for e in {
        let (x, y) = game.thief_coords();
        building_around_tile(x, y)
    }
    .into_iter()
    .filter_map(|(a, b)| game.building(a, b))
    .map(|&(_, player)| (player, game.player(player)))
    .filter(|&(id, player)| id != player_id && player.ressources().amounts() > 0)
    .map(|(id, player)| (id, player.name()))
    {
        if !players.contains(&e) {
            players.push(e);
        }
    }
    players.sort_by(|(a, _), (b, _)| a.cmp(b));
    players
}

#[profiling::function]
pub fn choose_steal(state: *mut State<Data, DataReturn>) -> Option<VecHStack> {
    let game = &unsafe { state.as_ref().expect("") }.data().game;
    let player_id = game.current_player_id();
    let players = steel_posibility(player_id, game);
    match players.as_slice() {
        [] => {
            unsafe { state.as_mut().expect("") }.mutate(|data| {
                data.game.set_thief_state(Thief::None);
            });
            None
        }
        [(pid, _)] => {
            unsafe { state.as_mut().expect("") }.mutate(|data| {
                data.game.steal(*pid, player_id);
                data.game.set_thief_state(Thief::None);
            });
            None
        }
        [_, _, ..] => Some(VecHStack::new(
            players
                .into_iter()
                .map(|(player, pname)| {
                    Box::new(CenterV::new(Margin::news(
                        Button::new(pname, state, move |data| {
                            data.game.steal(player, player_id);
                            data.game.set_thief_state(Thief::None);
                        }),
                        10.0,
                    ))) as Box<dyn Drawable>
                })
                .collect(),
        )),
    }
}

const GAME_ELEMENTS: usize = 155;
pub fn draw_map(state: &mut State<Data, DataReturn>) -> ZStack<GAME_ELEMENTS> {
    let mut r = Vec::<Box<dyn Drawable>>::with_capacity(GAME_ELEMENTS);
    r.push(Box::new(Space::new(500.0, 440.0)));
    for y in 0..5 {
        for x in 0..5 {
            if (x == 0 || x == 4) && (y == 0 || y == 4) {
                continue;
            }
            if x == 4 && (y == 1 || y == 3) {
                continue;
            }
            r.push(Box::new(tile::tile(x, y, state)));
        }
    }

    for (id, port) in state.data().game.ports().iter().enumerate() {
        r.push(Box::new(port::port(u8::try_from(id).expect(""), *port)));
    }

    for y in 0..5 {
        for x in 0..6 {
            if (y == 0 || y == 4) && (x == 0 || x == 5) {
                continue;
            }
            if (y == 1 || y == 3) && x == 5 {
                continue;
            }
            r.push(if let Some(vroad) = vroad(x, y, state) {
                Box::new(vroad)
            } else {
                Box::new(Empty::new())
            });
        }
    }

    for y in 0..6 {
        for x in 0..10 {
            if (y == 0 || y == 5) && (x <= 1 || x >= 8) {
                continue;
            }
            if (y == 1 || y == 4) && (x == 0 || x == 9) {
                continue;
            }
            r.push(if let Some(hroad) = hroad(x, y, state) {
                Box::new(hroad)
            } else {
                Box::new(Empty::new())
            });
        }
    }

    for y in 0..6 {
        for x in 0..11 {
            if (y == 0 || y == 5) && (x <= 1 || x >= 9) {
                continue;
            }
            if (y == 1 || y == 4) && (x == 0 || x == 10) {
                continue;
            }
            r.push(if let Some(building) = building(x, y, state) {
                Box::new(building)
            } else {
                Box::new(Empty::new())
            });
        }
    }
    ZStack::new(
        r.try_into()
            .ok()
            .unwrap_or_else(|| panic!("not {GAME_ELEMENTS} element")),
    )
}
