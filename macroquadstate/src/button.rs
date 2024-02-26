use macroquad::{prelude::*, ui::root_ui};

use crate::{drawable::Drawable, range::Range, state::DrawableState, state::State};

pub struct Button<
    'a,
    Data: DrawableState<DataReturn>,
    DataReturn: Drawable,
    Func: Fn(&mut Data) + Copy,
> {
    stop_draw: bool,
    text: &'a str,
    state: *mut State<Data, DataReturn>,
    action: Func,
}

impl<'a, Data: DrawableState<DataReturn>, DataReturn: Drawable, Func: Fn(&mut Data) + Copy>
    Button<'a, Data, DataReturn, Func>
{
    pub const fn new(text: &'a str, state: *mut State<Data, DataReturn>, action: Func) -> Self {
        Self {
            stop_draw: false,
            text,
            state,
            action,
        }
    }

    /// When you delete data who is currently used in the same frame.</br>
    /// Only use `new_stop` when `new` crash the app.
    pub const fn new_stop(
        text: &'a str,
        state: *mut State<Data, DataReturn>,
        action: Func,
    ) -> Self {
        Self {
            stop_draw: true,
            text,
            state,
            action,
        }
    }
}

#[profiling::all_functions]
impl<'a, Data: DrawableState<DataReturn>, DataReturn: Drawable, Func: Fn(&mut Data) + Copy> Drawable
    for Button<'a, Data, DataReturn, Func>
{
    fn width(&self) -> Range {
        if self.text.is_empty() {
            Range {
                min: 4.0,
                max: Some(4.0),
            }
        } else {
            let center = get_text_center(self.text, None, 15, 1.0, 0.0);
            let x = center.x * 2.13 + 4.0;
            Range {
                min: x,
                max: Some(x),
            }
        }
    }

    fn height(&self) -> Range {
        Range {
            min: 20.0,
            max: Some(20.0),
        }
    }

    fn draw(&mut self, x: f32, y: f32, _: f32, _: f32) -> Result<(), ()> {
        if root_ui().button(Vec2 { x, y }, self.text) {
            unsafe {
                self.state
                    .as_mut()
                    .expect("button state")
                    .mutate(self.action);
            }
            if self.stop_draw {
                Err(())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
