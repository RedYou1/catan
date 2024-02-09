use super::building;
use crate::{data::Data, player::Player};
use catan_lib::{
    game_manager::{self, Game},
    player::TPlayer,
};
use macroquad::{prelude::*, ui::root_ui};

#[profiling::function]
fn can_place_vroad(x: u8, y: u8, state: &Data) -> bool {
    let player_id = state.game.current_player_id();
    !if state.debut.road_turn() {
        game_manager::building_near_vroad(x, y)
            .iter()
            .any(|(x1, y1)| state.debut.near_building(*x1, *y1))
    } else {
        game_manager::hroad_near_vroad(x, y).iter().any(|(x1, y1)| {
            state
                .game
                .hroad(*x1, *y1)
                .map_or(false, |a| *a == player_id)
        })
    }
}

#[profiling::function]
pub fn vroad(x: u8, y: u8, starty: f32, state: &mut Data) {
    let off = y % 2;
    buy_button(
        (x, y),
        building::coords(x * 2 + off, y, starty),
        building::coords(x * 2 + off, y + 1, starty),
        state,
        can_place_vroad,
        Game::vroad,
        Game::vroad_mut,
    );
}

#[profiling::function]
fn can_place_hroad(x: u8, y: u8, state: &Data) -> bool {
    let player_id = state.game.current_player_id();
    !if state.debut.road_turn() {
        game_manager::building_near_hroad(x, y)
            .iter()
            .any(|(x1, y1)| state.debut.near_building(*x1, *y1))
    } else {
        game_manager::hroad_near_hroad(x, y).iter().any(|(x1, y1)| {
            state
                .game
                .hroad(*x1, *y1)
                .map_or(false, |a| *a == player_id)
        }) || game_manager::vroad_near_hroad(x, y).iter().any(|(x1, y1)| {
            state
                .game
                .vroad(*x1, *y1)
                .map_or(false, |a| *a == player_id)
        })
    }
}

#[profiling::function]
pub fn hroad(x: u8, y: u8, starty: f32, state: &mut Data) {
    buy_button(
        (x, y),
        building::coords(x, y, starty),
        building::coords(x + 1, y, starty),
        state,
        can_place_hroad,
        Game::hroad,
        Game::hroad_mut,
    );
}

fn buy_button<
    Get: Fn(&Game<Player, 4>, u8, u8) -> Option<&u8>,
    GetMut: Fn(&mut Game<Player, 4>, u8, u8) -> &mut Option<u8>,
    CanPlace: Fn(u8, u8, &Data) -> bool,
>(
    road_coord: (u8, u8),
    coord_1: (f32, f32),
    coord_2: (f32, f32),
    state: &mut Data,
    canplace: CanPlace,
    get: Get,
    get_mut: GetMut,
) {
    if let Some(&player_id) = get(&mut state.game, road_coord.0, road_coord.1) {
        draw_line(coord_1.0, coord_1.1, coord_2.0, coord_2.1, 15.0, BLACK);
        draw_line(
            coord_1.0,
            coord_1.1,
            coord_2.0,
            coord_2.1,
            10.0,
            state.game.player(player_id).color(),
        );
    } else {
        let ressource = state.game.current_player().ressources();
        if !state.debut.road_turn() && !ressource.can_buy(1, 0, 1, 0, 0) {
            return;
        }
        if canplace(road_coord.0, road_coord.1, state) {
            return;
        }
        let center_x = (coord_1.0 - coord_2.0) / 2.0 + coord_2.0;
        let center_y = (coord_1.1 - coord_2.1) / 2.0 + coord_2.1;
        draw_rectangle(center_x - 7.5, center_y - 12.5, 15.0, 25.0, BLACK);
        if !root_ui().button(
            Vec2 {
                x: center_x - 5.5,
                y: center_y - 10.0,
            },
            " ",
        ) {
            return;
        }
        buy_road(road_coord.0, road_coord.1, get_mut, state);
    }
}

#[profiling::function]
fn buy_road<GetMut: Fn(&mut Game<Player, 4>, u8, u8) -> &mut Option<u8>>(
    x: u8,
    y: u8,
    get_mut: GetMut,
    state: &mut Data,
) {
    *get_mut(&mut state.game, x, y) = Some(state.game.current_player_id());
    if state.debut.road_turn() {
        state.debut.place_road(&mut state.game);
    } else {
        state
            .game
            .current_player_mut()
            .ressources_mut()
            .buy(1, 0, 1, 0, 0);
    }
}
