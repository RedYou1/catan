use catan_lib::building::Building;
use macroquad::prelude::*;
use macroquadstate::{
    button::Button, empty::Empty, fix_circle::FixCircle, fix_rect::FixRect, offset::Offset,
    state::State, z_stack::ZStack, zstack,
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
pub fn building(x: u8, y: u8, state: &mut State<Data, DataReturn>) -> Option<ZStack<3>> {
    let data = state.data();
    let current_playing = data.game.current_player_id();
    let (center_x, center_y) = coords(x, y);

    match data.game.building(x, y) {
        Some((Building::BigHouse, player_id)) => Some(zstack![
            Empty::new(),
            Offset::new(
                center_x - 15.0,
                center_y - 15.0,
                FixCircle::new(15.0, BLACK),
            ),
            Offset::new(
                center_x - 12.5,
                center_y - 12.5,
                FixCircle::new(12.5, data.game.player(*player_id).color()),
            )
        ]),
        Some(&(Building::LittleHouse, player_id)) => Some(ZStack::new([
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
                if player_id == data.game.current_player_id()
                    && data.game.can_upgrade_building(player_id)
                {
                    Box::new(Offset::new(
                        center_x - 5.5,
                        center_y - 10.0,
                        Button::new(" ", state, move |data| {
                            data.game.upgrade_building(x, y, player_id);
                        }),
                    ))
                } else {
                    Box::new(Empty::new())
                }
            },
        ])),
        None => {
            if !data.game.can_place_building(x, y, current_playing) {
                return None;
            }
            Some(zstack![
                Empty::new(),
                Offset::new(
                    center_x - 7.5,
                    center_y - 12.5,
                    FixRect::new(15.0, 25.0, BLACK),
                ),
                Offset::new(
                    center_x - 5.5,
                    center_y - 10.0,
                    Button::new(" ", state, move |data| {
                        data.game.buy_building(x, y);
                    }),
                )
            ])
        }
    }
}
