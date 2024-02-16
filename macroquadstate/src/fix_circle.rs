use macroquad::prelude::*;

use crate::{drawable::Drawable, range::Range};

pub struct FixCircle {
    radius: f32,
    color: Color,
}

impl FixCircle {
    pub fn new(radius: f32, color: Color) -> Self {
        Self { radius, color }
    }
}

#[profiling::all_functions]
impl Drawable for FixCircle {
    fn width(&self) -> Range {
        Range {
            min: self.radius,
            max: Some(self.radius),
        }
    }

    fn height(&self) -> Range {
        Range {
            min: self.radius,
            max: Some(self.radius),
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let radius = f32::min(width, height);
        draw_circle(x, y, radius, self.color)
    }
}
