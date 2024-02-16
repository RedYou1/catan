use crate::{drawable::Drawable, range::Range};

pub struct Space {
    width: f32,
    height: f32,
}

impl Space {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[profiling::all_functions]
impl Drawable for Space {
    fn width(&self) -> Range {
        Range {
            min: self.width,
            max: Some(self.width),
        }
    }

    fn height(&self) -> Range {
        Range {
            min: self.height,
            max: Some(self.height),
        }
    }

    fn draw(&mut self, _: f32, _: f32, _: f32, _: f32) {}
}
