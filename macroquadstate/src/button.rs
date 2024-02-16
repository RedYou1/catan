use macroquad::{prelude::*, ui::root_ui};

use crate::{drawable::Drawable, range::Range, state::DrawableState, state::State};

pub struct Button<'a, Data: DrawableState<DataReturn>, DataReturn: Drawable, Func: FnMut(&mut Data)>
{
    text: &'a str,
    state: *mut State<Data, DataReturn>,
    action: Func,
}

impl<'a, Data: DrawableState<DataReturn>, DataReturn: Drawable, Func: FnMut(&mut Data)>
    Button<'a, Data, DataReturn, Func>
{
    pub fn new(text: &'a str, state: *mut State<Data, DataReturn>, action: Func) -> Self {
        Self {
            text,
            state,
            action,
        }
    }
}

#[profiling::all_functions]
impl<'a, Data: DrawableState<DataReturn>, DataReturn: Drawable, Func: FnMut(&mut Data)> Drawable
    for Button<'a, Data, DataReturn, Func>
{
    fn width(&self) -> Range {
        let center = get_text_center(self.text, None, 15, 1.0, 0.0);
        let x = center.x * 2.0 + 11.0;
        Range {
            min: x,
            max: Some(x),
        }
    }

    fn height(&self) -> Range {
        let center = get_text_center(self.text, None, 15, 1.0, 0.0);
        let y = center.y * -2.0 + 18.0;
        Range {
            min: y,
            max: Some(y),
        }
    }

    fn draw(&mut self, x: f32, y: f32, _: f32, _: f32) {
        if root_ui().button(Vec2 { x, y: y + 2.0 }, self.text) {
            unsafe {
                self.state
                    .as_mut()
                    .expect("button state")
                    .mutate(&mut self.action);
            }
        }
    }
}
