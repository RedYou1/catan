use catan_lib::ressource::Ressource;
use macroquad::prelude::*;
use macroquadstate::{
    button::Button,
    center::{Center, CenterH},
    empty::Empty,
    fix_hex::FixHex,
    fix_rect::FixRect,
    fix_text::FixText,
    margin::Margin,
    offset::Offset,
    state::State,
    v_stack::VStack,
    vstack,
    z_stack::ZStack,
};

use crate::{
    data::{Data, DataReturn, Thief},
    HEX_SIZE,
};

#[profiling::function]
pub fn tile(x: u8, y: u8, state: &mut State<Data, DataReturn>) -> Offset<ZStack<3>> {
    let tile = state.data().game.tiles()[y as usize][x as usize];
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

    let center_x = 1.8 * HEX_SIZE * (px + isoff) + 4.0 * HEX_SIZE;
    let center_y = 1.54 * HEX_SIZE * py + HEX_SIZE * 0.25;

    Offset::new(
        center_x,
        center_y,
        ZStack::new([
            Box::new(FixHex::new(HEX_SIZE, color)),
            if let Some(tile) = tile {
                Box::new(Center::new(vstack![
                    CenterH::new(Margin::new(
                        FixText::new(
                            tile.dice_id().to_string(),
                            25,
                            if tile.dice_id() == 6 || tile.dice_id() == 8 {
                                DARKBLUE
                            } else {
                                BLACK
                            },
                        ),
                        0.0,
                        0.0,
                        0.0,
                        5.0,
                    )),
                    CenterH::new(Margin::new(
                        FixText::new(
                            str::repeat(
                                "*",
                                usize::try_from(6 - (7 - i32::from(tile.dice_id())).abs())
                                    .expect("number try_from"),
                            ),
                            25,
                            if tile.dice_id() == 6 || tile.dice_id() == 8 {
                                DARKBLUE
                            } else {
                                BLACK
                            },
                        ),
                        0.0,
                        0.0,
                        0.0,
                        5.0,
                    ))
                ]))
            } else {
                Box::new(Empty::new())
            },
            if *state.data().game.thief() == (x, y) {
                Box::new(Center::new(FixRect::new(HEX_SIZE, HEX_SIZE, BLACK)))
            } else if state.data().thief == Thief::Waiting {
                Box::new(Center::new(Button::new(" ", state, move |data| {
                    data.thief = Thief::Choosing;
                    *data.game.thief_mut() = (x, y);
                })))
            } else {
                Box::new(Empty::new())
            },
        ]),
    )
}
