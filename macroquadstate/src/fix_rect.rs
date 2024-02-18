use macroquad::prelude::*;

use crate::{drawable::Drawable, range::Range};

pub struct FixRect {
    width: f32,
    height: f32,
    color: Color,
}

impl FixRect {
    
    pub const fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
        }
    }
}

#[profiling::all_functions]
impl Drawable for FixRect {
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

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        draw_rectangle(x, y, width, height, self.color);
    }
}
