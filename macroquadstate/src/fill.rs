use macroquad::prelude::*;

use crate::{drawable::Drawable, range::Range};

pub struct Fill {
    min_width: f32,
    min_height: f32,
    color: Color,
}

impl Fill {
    
    pub const fn new(min_width: f32, min_height: f32, color: Color) -> Self {
        Self {
            min_width,
            min_height,
            color,
        }
    }
}

#[profiling::all_functions]
impl Drawable for Fill {
    fn width(&self) -> Range {
        Range {
            min: self.min_width,
            max: None,
        }
    }

    fn height(&self) -> Range {
        Range {
            min: self.min_height,
            max: None,
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        draw_rectangle(x, y, width, height, self.color);
    }
}
