use catan_lib::{
    game_manager::building_around_tile, player::TPlayer, ressource_manager::RessourceManager,
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
    data::{Data, DataReturn, GamePage, Thief},
    draw::{
        building::building,
        port,
        road::{hroad, vroad},
        tile,
    },
};

#[profiling::function]
pub fn game(state: &mut State<Data, DataReturn>) -> VStack<4> {
    let data = state.data();
    let current_player = data.game.current_player();

    let to_choose: (u8, u8, u8) = {
        if data.debut.is_starting() {
            (0, 0, 0)
        } else if let Some((a, b)) = data.dices {
            if data.thief == Thief::Waiting {
                (0, a, b)
            } else if data.thief == Thief::Choosing {
                (1, a, b)
            } else {
                (2, a, b)
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
                CenterH::new(FixText::new(format!("{a} {b}"), 25, WHITE)),
                CenterH::new(Button::new("Next", state, |data| {
                    data.game.next_player();
                    data.dices = None;
                }))
            ])),
            (3, _, _) => Box::new(CenterH::new(Button::new("Dice", state, |data| {
                let (a, b) = data.game.throw_dice();
                data.dices = Some((a, b));
                data.thief = if a + b == 7 {
                    Thief::Waiting
                } else {
                    Thief::None
                };
                if a + b == 7 {
                    data.to_reduce = RessourceManager::default();
                    data.page = GamePage::Reduce;
                }
            }))),
            _ => Box::new(Empty::new()),
        },
    ])
}

#[profiling::function]
pub fn choose_steal(state: *mut State<Data, DataReturn>) -> Option<VecHStack> {
    let data = unsafe { state.as_ref().expect("") }.data();
    let player_id = data.game.current_player_id();
    let mut players: Vec<(u8, &str)> = Vec::with_capacity(data.game.players_len() as usize);
    for e in building_around_tile(data.game.thief().0, data.game.thief().1)
        .iter()
        .filter_map(|(a, b)| data.game.building(*a, *b))
        .map(|(_, player)| (*player, data.game.player(*player)))
        .filter(|(id, player)| *id != player_id && player.ressources().amounts() > 0)
        .map(|(id, player)| (id, player.name()))
    {
        if !players.contains(&e) {
            players.push(e);
        }
    }
    if players.is_empty() {
        unsafe { state.as_mut().expect("") }.mutate(|data| {
            data.thief = Thief::None;
        });
        None
    } else if players.len() == 1 {
        unsafe { state.as_mut().expect("") }.mutate(|data| {
            data.game.steal(players[0].0, player_id);
            data.thief = Thief::None;
        });
        None
    } else {
        Some(VecHStack::new(
            players
                .iter()
                .map(|&(player, pname)| {
                    Box::new(CenterV::new(Button::new(pname, state, move |data| {
                        data.game.steal(player, player_id);
                        data.thief = Thief::None;
                    }))) as Box<dyn Drawable>
                })
                .collect(),
        ))
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
