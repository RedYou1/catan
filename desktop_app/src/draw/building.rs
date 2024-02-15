use catan_lib::{building::Building, game_manager, player::TPlayer};
use macroquad::prelude::*;
use macroquadstate::{
    button::Button, fix_circle::FixCircle, fix_rect::FixRect, offset::Offset, space::Space,
    state::State, z_stack::ZStack,
};

use crate::{
    data::{Data, DataReturn},
    HEX_SIZE,
};

#[profiling::function]
pub fn coords(x: u8, y: u8) -> (f32, f32) {
    let px = f32::from(x) - 5.0;
    let py = f32::from(y);
    let isoff = f32::from(x % 2 == y % 2);
    (
        1.8 * HEX_SIZE * px / 2.0 + 5.0 * HEX_SIZE,
        HEX_SIZE * 0.25 + 1.54 * HEX_SIZE * py + 0.5 * HEX_SIZE * isoff,
    )
}

#[profiling::function]
pub fn building(x: u8, y: u8, state: &mut State<Data, DataReturn>) -> ZStack {
    let data = state.data();
    let current_playing = data.game.current_player_id();
    let (center_x, center_y) = coords(x, y);

    match data.game.building(x, y) {
        Some((Building::BigHouse, player_id)) => ZStack::new(vec![
            Box::new(Offset::new(
                center_x - 15.0,
                center_y - 15.0,
                FixCircle::new(15.0, BLACK),
            )),
            Box::new(Offset::new(
                center_x - 12.5,
                center_y - 12.5,
                FixCircle::new(12.5, data.game.player(*player_id).color()),
            )),
        ]),
        Some(&(Building::LittleHouse, player_id)) => ZStack::new(vec![
            Box::new(Offset::new(
                center_x - 10.0,
                center_y - 15.0,
                FixRect::new(20.0, 30.0, BLACK),
            )),
            Box::new(Offset::new(
                center_x - 7.5,
                center_y - 12.5,
                FixRect::new(15.0, 25.0, data.game.player(player_id).color()),
            )),
            {
                if player_id == data.game.current_player_id() {
                    let ressource = data.game.current_player().ressources();
                    if ressource.can_buy(0, 2, 0, 0, 3) {
                        Box::new(Offset::new(
                            center_x - 5.5,
                            center_y - 10.0,
                            Button::new(" ", state, move |data| {
                                upgrade(x, y, player_id, data);
                            }),
                        ))
                    } else {
                        Box::new(Space::new(0.0, 0.0))
                    }
                } else {
                    Box::new(Space::new(0.0, 0.0))
                }
            },
        ]),
        None => {
            let ressource = data.game.current_player().ressources();
            if !data.debut.building_turn() && !ressource.can_buy(1, 1, 1, 1, 0) {
                return ZStack::new(vec![]);
            }
            if can_place(x, y, current_playing, data) {
                return ZStack::new(vec![]);
            }
            if data.game.building_in_range(x, y) {
                return ZStack::new(vec![]);
            }
            ZStack::new(vec![
                Box::new(Offset::new(
                    center_x - 7.5,
                    center_y - 12.5,
                    FixRect::new(15.0, 25.0, BLACK),
                )),
                Box::new(Offset::new(
                    center_x - 5.5,
                    center_y - 10.0,
                    Button::new(" ", state, move |data| {
                        buy_none(x, y, data);
                    }),
                )),
            ])
        }
    }
}

#[profiling::function]
fn upgrade(x: u8, y: u8, player_id: u8, state: &mut Data) {
    *state.game.building_mut(x, y) = Some((Building::BigHouse, player_id));
    state
        .game
        .current_player_mut()
        .ressources_mut()
        .buy(0, 2, 0, 0, 3);
}

#[profiling::function]
fn can_place(x: u8, y: u8, current_playing: u8, state: &Data) -> bool {
    state.game.building_in_range(x, y)
        || !(state.debut.building_turn()
            || game_manager::hroad_near_building(x, y)
                .iter()
                .any(|(x1, y1)| {
                    state
                        .game
                        .hroad(*x1, *y1)
                        .map_or(false, |a| *a == current_playing)
                })
            || game_manager::vroad_near_building(x, y)
                .iter()
                .any(|(x1, y1)| {
                    state
                        .game
                        .vroad(*x1, *y1)
                        .map_or(false, |a| *a == current_playing)
                }))
}

#[profiling::function]
fn buy_none(x: u8, y: u8, state: &mut Data) {
    *state.game.building_mut(x, y) = Some((Building::LittleHouse, state.game.current_player_id()));
    if state.debut.building_turn() {
        state.debut.place_building(x, y);
    } else {
        state
            .game
            .current_player_mut()
            .ressources_mut()
            .buy(1, 1, 1, 1, 0);
    }
}
