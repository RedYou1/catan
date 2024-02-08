use catan_lib::{game_manager::Game, ressource::Ressource};
use macroquad::{prelude::*, ui::root_ui};

use crate::{
    draw::texts_vertical::texts_vertical,
    player::Player,
    state::{State, Thief},
    HEX_SIZE,
};

pub fn tile(x: u8, y: u8, starty: f32, game: &mut Game<Player, 4>, state: &mut State) {
    let tile = game.tiles()[y as usize][x as usize];
    let color = if let Some(tile) = tile {
        match tile.ressource() {
            Ressource::Tree => GREEN,
            Ressource::Wheet => GOLD,
            Ressource::Brick => RED,
            Ressource::Sheep => LIGHTGRAY,
            Ressource::Stone => GRAY,
        }
    } else {
        YELLOW
    };

    let px = f32::from(x) - 2.0;
    let py = f32::from(y);
    let isoff = f32::from(y % 2) / 2.0;

    let center_x = screen_width() / 2.0 + 1.8 * HEX_SIZE * (px + isoff);
    let center_y = starty + HEX_SIZE * 2.0 + 1.54 * HEX_SIZE * py;

    draw_hexagon(center_x, center_y, HEX_SIZE, 0.0, true, BLANK, color);

    if let Some(tile) = tile {
        texts_vertical(
            &[
                tile.dice_id().to_string(),
                str::repeat(
                    "*",
                    usize::try_from(6 - (7 - i32::from(tile.dice_id())).abs())
                        .expect("number try_from"),
                ),
            ],
            center_x,
            center_y - HEX_SIZE / 4.0,
            5.0,
            25,
            if tile.dice_id() == 6 || tile.dice_id() == 8 {
                DARKBLUE
            } else {
                BLACK
            },
        );
    }

    if *game.thief() == (x, y) {
        draw_rectangle(
            center_x - HEX_SIZE / 2.0,
            center_y - HEX_SIZE / 2.0,
            HEX_SIZE,
            HEX_SIZE,
            BLACK,
        );
    } else if state.thief == Thief::Waiting
        && root_ui().button(
            Vec2 {
                x: center_x - 5.5,
                y: center_y - 10.0,
            },
            " ",
        )
    {
        state.thief = Thief::Choosing;
        *game.thief_mut() = (x, y);
    }
}
