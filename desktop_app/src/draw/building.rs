use catan_lib::{building::Building, game_manager, player::TPlayer};
use macroquad::{prelude::*, ui::root_ui};

use crate::{data::Data, HEX_SIZE};

pub fn coords(x: u8, y: u8, starty: f32) -> (f32, f32) {
    let px = f32::from(x) - 5.0;
    let py = f32::from(y);
    let isoff = f32::from(x % 2 == y % 2);
    (
        screen_width() / 2.0 + 1.8 * HEX_SIZE * px / 2.0,
        starty + HEX_SIZE * 1.0 + 1.54 * HEX_SIZE * py + 0.5 * HEX_SIZE * isoff,
    )
}

pub fn building(x: u8, y: u8, starty: f32, state: &mut Data) {
    let current_playing = state.game.current_player_id();
    let (center_x, center_y) = coords(x, y, starty);

    match state.game.building(x, y) {
        Some((Building::BigHouse, player_id)) => {
            draw_circle(center_x, center_y, 15.0, BLACK);
            draw_circle(
                center_x,
                center_y,
                12.5,
                state.game.player(*player_id).color(),
            );
        }
        Some((Building::LittleHouse, player_id)) => {
            draw_rectangle(center_x - 10.0, center_y - 15.0, 20.0, 30.0, BLACK);
            draw_rectangle(
                center_x - 7.5,
                center_y - 12.5,
                15.0,
                25.0,
                state.game.player(*player_id).color(),
            );
            if *player_id != state.game.current_player_id() {
                return;
            }
            let ressource = state.game.current_player().ressources();
            if !ressource.can_buy(0, 2, 0, 0, 3) {
                return;
            }
            if !root_ui().button(
                Vec2 {
                    x: center_x - 5.5,
                    y: center_y - 10.0,
                },
                " ",
            ) {
                return;
            }
            *state.game.building_mut(x, y) = Some((Building::BigHouse, *player_id));
            state
                .game
                .current_player_mut()
                .ressources_mut()
                .buy(0, 2, 0, 0, 3);
        }
        None => {
            let ressource = state.game.current_player().ressources();
            if !state.debut.building_turn() && !ressource.can_buy(1, 1, 1, 1, 0) {
                return;
            }
            if state.game.building_in_range(x, y)
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
            {
                return;
            }
            if state.game.building_in_range(x, y) {
                return;
            }
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
            *state.game.building_mut(x, y) =
                Some((Building::LittleHouse, state.game.current_player_id()));
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
    }
}
