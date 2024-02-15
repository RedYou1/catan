use catan_lib::{
    game_manager::building_around_tile, player::TPlayer, ressource_manager::RessourceManager,
};
use macroquad::prelude::*;
use macroquadstate::{
    button::Button,
    center::{CenterH, CenterV},
    drawable::Drawable,
    fix_text::FixText,
    h_stack::HStack,
    margin::Margin,
    space::Space,
    state::State,
    v_stack::VStack,
    z_stack::ZStack,
};

use crate::{
    data::{Data, DataReturn, Thief},
    draw::{
        building::building,
        road::{hroad, vroad},
        tile,
    },
    Page,
};

pub fn game(state: &mut State<Data, DataReturn>) -> VStack {
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
    VStack::new(vec![
        Box::new(CenterH::new(Margin::news(
            FixText::new(
                format!("Player to play: {}", current_player.name()),
                25,
                WHITE,
            ),
            2.5,
        ))),
        Box::new(CenterH::new(VStack::new(
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
                            WHITE,
                        ),
                        2.0,
                    )) as Box<dyn Drawable>
                })
                .collect(),
        ))),
        Box::new(CenterH::new(Margin::news(draw_map(state), 10.0))),
        match to_choose {
            (1, _, _) => Box::new(CenterH::new(choose_steal(state))),
            (2, a, b) => Box::new(CenterH::new(VStack::new(vec![
                Box::new(CenterH::new(FixText::new(format!("{a} {b}"), 25, WHITE))),
                Box::new(CenterH::new(Button::new("Next", state, |data| {
                    data.game.next_player();
                    data.dices = None;
                }))),
            ]))),
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
                    data.page = Page::Reduce;
                }
            }))),
            _ => Box::new(Space::new(0.0, 0.0)),
        },
    ])
}

#[profiling::function]
pub fn choose_steal(state: &mut State<Data, DataReturn>) -> HStack {
    let data = state.data();
    let player_id = data.game.current_player_id();
    let mut players: Vec<(u8, &'static str)> = Vec::with_capacity(data.game.players().len());
    for e in building_around_tile(data.game.thief().0, data.game.thief().1)
        .iter()
        .filter_map(|(a, b)| data.game.building(*a, *b))
        .map(|(_, player)| (*player, *data.game.player(*player)))
        .filter(|(id, player)| *id != player_id && player.ressources().amounts() > 0)
        .map(|(id, player)| (id, player.name()))
    {
        if !players.contains(&e) {
            players.push(e);
        }
    }
    if players.is_empty() {
        state.mutate(&mut |data| {
            data.thief = Thief::None;
        });
        HStack::new(vec![])
    } else if players.len() == 1 {
        state.mutate(&mut |data| {
            data.game.steal(players[0].0, player_id);
            data.thief = Thief::None;
        });
        HStack::new(vec![])
    } else {
        HStack::new(
            players
                .iter()
                .map(|&(player, pname)| {
                    Box::new(CenterV::new(Button::new(pname, state, move |data| {
                        data.game.steal(player, player_id);
                        data.thief = Thief::None;
                    }))) as Box<dyn Drawable>
                })
                .collect(),
        )
    }
}

pub fn draw_map(state: &mut State<Data, DataReturn>) -> ZStack {
    let mut r = Vec::<Box<dyn Drawable>>::with_capacity(145);
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

    for y in 0..6 {
        for x in 0..11 {
            if (y == 0 || y == 5) && (x <= 1 || x >= 9) {
                continue;
            }
            if (y == 1 || y == 4) && (x == 0 || x == 10) {
                continue;
            }
            r.push(Box::new(building(x, y, state)));
        }
    }

    for y in 0..5 {
        for x in 0..6 {
            if (y == 0 || y == 4) && (x == 0 || x == 5) {
                continue;
            }
            if (y == 1 || y == 3) && x == 5 {
                continue;
            }
            r.push(Box::new(vroad(x, y, state)));
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
            r.push(Box::new(hroad(x, y, state)));
        }
    }
    assert_eq!(r.len(), 145);
    ZStack::new(r)
}
