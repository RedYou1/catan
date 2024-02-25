use macroquad::color::WHITE;
use macroquadstate::{
    button::Button,
    center::{CenterH, CenterV},
    fix_text::FixText,
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
            Wrapper::new(CenterH::new(Button::new("+", state, |data| {
                data.page.unwrap_main_mut().player_number += 1;
            })))
        } else {
            Wrapper::new(Space::new(0.0, 20.0))
        },
        CenterH::new(FixText::new(format!("{}", data.player_number), 25, WHITE)),
        if data.player_number > 2 {
            Wrapper::new(CenterH::new(Button::new("-", state, |data| {
                data.page.unwrap_main_mut().player_number -= 1;
            })))
        } else {
            Wrapper::new(Space::new(0.0, 20.0))
        },
        CenterH::new(Button::new("PLAY", state, |data| {
            data.page = Page::Game(State::new(Data::new(data.page.unwrap_main().player_number)));
        }))
    ])
}
