use catan_lib::{ressource::Ressource, tile::Tile};
use macroquad::prelude::*;

use crate::{draw::texts_vertical::texts_vertical, HEX_SIZE};

pub fn tile(x: usize, y: usize, starty: f32, tile: Option<Tile>) {
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

    let px = f32::from(i16::try_from(x).expect("number try_from")) - 2.0;
    let py = f32::from(i16::try_from(y).expect("number try_from"));
    let isoff = f32::from(u8::try_from(y % 2).expect("number try_from")) / 2.0;

    draw_hexagon(
        screen_width() / 2.0 + 1.8 * HEX_SIZE * (px + isoff),
        starty + HEX_SIZE * 2.0 + 1.54 * HEX_SIZE * py,
        HEX_SIZE,
        0.0,
        true,
        BLANK,
        color,
    );

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
            screen_width() / 2.0 + 1.8 * HEX_SIZE * (px + isoff),
            starty + HEX_SIZE * 2.0 + 1.54 * HEX_SIZE * py - HEX_SIZE / 4.0,
            5.0,
            25,
            if tile.dice_id() == 6 || tile.dice_id() == 8 {
                DARKBLUE
            } else {
                BLACK
            },
        );
    }
}
