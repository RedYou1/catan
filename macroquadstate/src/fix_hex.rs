use macroquad::prelude::*;

use crate::{drawable::Drawable, range::Range};

pub struct FixHex {
    radius: f32,
    color: Color,
}

impl FixHex {
    pub const fn new(radius: f32, color: Color) -> Self {
        Self { radius, color }
    }
}

#[profiling::all_functions]
impl Drawable for FixHex {
    fn width(&self) -> Range {
        Range {
            min: self.radius * 2.0,
            max: Some(self.radius * 2.0),
        }
    }

    fn height(&self) -> Range {
        Range {
            min: self.radius * 2.0,
            max: Some(self.radius * 2.0),
        }
    }

    fn draw(&mut self, x: f32, y: f32, _: f32, _: f32) -> Result<(), ()> {
        draw_hexagon(
            x + self.radius,
            y + self.radius,
            self.radius,
            0.0,
            true,
            BLANK,
            self.color,
        );
        Ok(())
    }
}
