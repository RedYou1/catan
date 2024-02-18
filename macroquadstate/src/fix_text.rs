use macroquad::prelude::*;

use crate::{drawable::Drawable, range::Range};

pub struct FixText {
    text: String,
    font: u16,
    color: Color,
    width: Range,
    height: Range,
}

impl FixText {
    
    pub fn new(text: String, font: u16, color: Color) -> Self {
        let center = get_text_center(text.as_str(), None, font, 1.0, 0.0);
        let width = {
            let x = center.x * 2.0;
            Range {
                min: x,
                max: Some(x),
            }
        };
        let height = {
            let y = center.y * -2.0;
            Range {
                min: y,
                max: Some(y),
            }
        };
        Self {
            text,
            font,
            color,
            width,
            height,
        }
    }
}

#[profiling::all_functions]
impl Drawable for FixText {
    fn width(&self) -> Range {
        self.width.clone()
    }

    fn height(&self) -> Range {
        self.height.clone()
    }

    fn draw(&mut self, x: f32, y: f32, _: f32, _: f32) {
        draw_text(
            self.text.as_str(),
            x,
            y + self.height.min,
            f32::from(self.font),
            self.color,
        );
    }
}
