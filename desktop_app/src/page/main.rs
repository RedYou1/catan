use macroquad::color::WHITE;
use macroquadstate::{
    button::Button,
    center::{CenterH, CenterV},
    fix_text::FixText,
    margin::Margin,
    space::Space,
    state::State,
    v_stack::VStack,
    vstack,
    wrapper::Wrapper,
};

use crate::{data::Data, MainData, Page, Window, WindowReturn};

pub fn main(data: &MainData, state: &mut State<Window, WindowReturn>) -> CenterV<VStack<4>> {
    CenterV::new(vstack![
        if data.player_number < 8 {
            Wrapper::new(CenterH::new(Margin::news(
                Button::new("+", state, |data| {
                    data.page.unwrap_main_mut().player_number += 1;
                }),
                2.5,
            )))
        } else {
            Wrapper::new(Space::new(0.0, 15.0))
        },
        CenterH::new(Margin::news(
            FixText::new(format!("{}", data.player_number), 25, WHITE),
            5.0
        )),
        if data.player_number > 2 {
            Wrapper::new(CenterH::new(Margin::news(
                Button::new("-", state, |data| {
                    data.page.unwrap_main_mut().player_number -= 1;
                }),
                2.5,
            )))
        } else {
            Wrapper::new(Space::new(0.0, 15.0))
        },
        CenterH::new(Margin::news(
            Button::new("PLAY", state, |data| {
                data.page =
                    Page::Game(State::new(Data::new(data.page.unwrap_main().player_number)));
            }),
            5.0
        ))
    ])
}
