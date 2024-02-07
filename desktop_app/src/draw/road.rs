use super::building;
use crate::{player::Player, Starting};
use catan_lib::{
    game_manager::{self, Game},
    player::TPlayer,
};
use macroquad::{prelude::*, ui::root_ui};

fn can_place_vroad(x: usize, y: usize, game: &Game<Player, 4>, debut: &Starting) -> bool {
    let player_id = game.current_player_id();
    if debut.road_turn() {
        !game_manager::building_near_vroad(x, y)
            .iter()
            .any(|(x1, y1)| debut.near_building(*x1, *y1))
    } else {
        !(game_manager::building_near_vroad(x, y)
            .iter()
            .any(|(x1, y1)| game.building(*x1, *y1).map_or(false, |a| a.1 == player_id))
            || game_manager::hroad_near_vroad(x, y)
                .iter()
                .any(|(x1, y1)| game.hroad(*x1, *y1).map_or(false, |a| *a == player_id)))
    }
}

pub fn vroad(x: usize, y: usize, starty: f32, game: &mut Game<Player, 4>, debut: &mut Starting) {
    let off = y % 2;
    buy_button(
        (x, y),
        building::coords(x * 2 + off, y, starty),
        building::coords(x * 2 + off, y + 1, starty),
        game,
        &mut (debut, can_place_vroad),
        Game::vroad,
        Game::vroad_mut,
    );
}

fn can_place_hroad(x: usize, y: usize, game: &Game<Player, 4>, debut: &Starting) -> bool {
    let player_id = game.current_player_id();
    if debut.road_turn() {
        !game_manager::building_near_hroad(x, y)
            .iter()
            .any(|(x1, y1)| debut.near_building(*x1, *y1))
    } else {
        !(game_manager::building_near_hroad(x, y)
            .iter()
            .any(|(x1, y1)| game.building(*x1, *y1).map_or(false, |a| a.1 == player_id))
            || game_manager::hroad_near_hroad(x, y)
                .iter()
                .any(|(x1, y1)| game.hroad(*x1, *y1).map_or(false, |a| *a == player_id))
            || game_manager::vroad_near_hroad(x, y)
                .iter()
                .any(|(x1, y1)| game.vroad(*x1, *y1).map_or(false, |a| *a == player_id)))
    }
}

pub fn hroad(x: usize, y: usize, starty: f32, game: &mut Game<Player, 4>, debut: &mut Starting) {
    buy_button(
        (x, y),
        building::coords(x, y, starty),
        building::coords(x + 1, y, starty),
        game,
        &mut (debut, can_place_hroad),
        Game::hroad,
        Game::hroad_mut,
    );
}

fn buy_button<
    Get: Fn(&Game<Player, 4>, usize, usize) -> Option<&usize>,
    GetMut: Fn(&mut Game<Player, 4>, usize, usize) -> &mut Option<usize>,
    CanPlace: Fn(usize, usize, &Game<Player, 4>, &Starting) -> bool,
>(
    road_coord: (usize, usize),
    coord_1: (f32, f32),
    coord_2: (f32, f32),
    game: &mut Game<Player, 4>,
    option: &mut (&mut Starting, CanPlace),
    get: Get,
    get_mut: GetMut,
) {
    if let Some(player_id) = get(game, road_coord.0, road_coord.1) {
        draw_line(coord_1.0, coord_1.1, coord_2.0, coord_2.1, 15.0, BLACK);
        draw_line(
            coord_1.0,
            coord_1.1,
            coord_2.0,
            coord_2.1,
            10.0,
            game.player(*player_id).color(),
        );
    } else {
        let ressource = game.current_player().ressources();
        if !option.0.road_turn() && !ressource.can_buy(1, 0, 1, 0, 0) {
            return;
        }
        if option.1(road_coord.0, road_coord.1, game, option.0) {
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
        *get_mut(game, road_coord.0, road_coord.1) = Some(game.current_player_id());
        if option.0.road_turn() {
            option.0.place_road(game);
        } else {
            game.current_player_mut()
                .ressources_mut()
                .buy(1, 0, 1, 0, 0);
        }
    }
}
