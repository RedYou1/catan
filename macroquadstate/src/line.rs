use macroquad::prelude::*;

use crate::{drawable::Drawable, range::Range};

pub struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    thickness: f32,
    color: Color,
}

impl Line {
    
    pub const fn new(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            thickness,
            color,
        }
    }
}

#[profiling::all_functions]
impl Drawable for Line {
    fn width(&self) -> Range {
        let w = (self.x2 - self.x1).abs();
        Range {
            min: w,
            max: Some(w),
        }
    }

    fn height(&self) -> Range {
        let h = (self.y2 - self.y1).abs();
        Range {
            min: h,
            max: Some(h),
        }
    }

    fn draw(&mut self, x: f32, y: f32, _: f32, _: f32) {
        draw_line(
            self.x1 + x,
            self.y1 + y,
            self.x2 + x,
            self.y2 + y,
            self.thickness,
            self.color,
        );
    }
}
