use crate::{drawable::Drawable, range::Range};

pub struct Empty;

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self {}
    }
}

#[profiling::all_functions]
impl Drawable for Empty {
    fn width(&self) -> Range {
        Range {
            min: 0.0,
            max: Some(0.0),
        }
    }

    fn height(&self) -> Range {
        Range {
            min: 0.0,
            max: Some(0.0),
        }
    }

    fn draw(&mut self, _: f32, _: f32, _: f32, _: f32) {}
}
