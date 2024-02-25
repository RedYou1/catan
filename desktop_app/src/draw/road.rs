use super::building;
use crate::{
    data::{Data, DataReturn},
    player::Player,
};
use catan_lib::game_manager::Game;
use macroquad::prelude::*;
use macroquadstate::{
    button::Button, fix_rect::FixRect, line::Line, offset::Offset, state::State, z_stack::ZStack,
    zstack,
};

#[profiling::function]
pub fn vroad(x: u8, y: u8, state: &mut State<Data, DataReturn>) -> Option<ZStack<2>> {
    let off = y % 2;
    buy_button(
        (x, y),
        building::coords(x * 2 + off, y),
        building::coords(x * 2 + off, y + 1),
        state,
        Game::can_place_vroad,
        Game::vroad,
        &Game::buy_vroad,
    )
}

#[profiling::function]
pub fn hroad(x: u8, y: u8, state: &mut State<Data, DataReturn>) -> Option<ZStack<2>> {
    buy_button(
        (x, y),
        building::coords(x, y),
        building::coords(x + 1, y),
        state,
        Game::can_place_hroad,
        Game::hroad,
        &Game::buy_hroad,
    )
}

fn buy_button<
    Get: Fn(&Game<Player>, u8, u8) -> Option<&u8>,
    Buy: Fn(&mut Game<Player>, u8, u8),
    CanPlace: Fn(&Game<Player>, u8, u8) -> bool,
>(
    road_coord: (u8, u8),
    coord_1: (f32, f32),
    coord_2: (f32, f32),
    state: &mut State<Data, DataReturn>,
    canplace: CanPlace,
    get: Get,
    buy: &'static Buy,
) -> Option<ZStack<2>> {
    let game = &state.data().game;
    if let Some(&player_id) = get(game, road_coord.0, road_coord.1) {
        Some(zstack![
            Line::new(coord_1.0, coord_1.1, coord_2.0, coord_2.1, 15.0, BLACK),
            Line::new(
                coord_1.0,
                coord_1.1,
                coord_2.0,
                coord_2.1,
                10.0,
                game.player(player_id).color(),
            )
        ])
    } else if canplace(&state.data().game, road_coord.0, road_coord.1) {
        let center_x = (coord_1.0 - coord_2.0) / 2.0 + coord_2.0;
        let center_y = (coord_1.1 - coord_2.1) / 2.0 + coord_2.1;
        Some(zstack![
            Offset::new(
                center_x - 7.5,
                center_y - 12.5,
                FixRect::new(15.0, 25.0, BLACK),
            ),
            Offset::new(
                center_x - 5.5,
                center_y - 10.0,
                Button::new(" ", state, move |data| {
                    buy(&mut data.game, road_coord.0, road_coord.1);
                }),
            )
        ])
    } else {
        None
    }
}
