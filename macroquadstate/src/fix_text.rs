use macroquad::prelude::*;

use crate::{drawable::Drawable, range::Range};

pub struct FixText {
    text: String,
    font: u16,
    color: Color,
}

impl FixText {
    pub fn new(text: String, font: u16, color: Color) -> Self {
        Self { text, font, color }
    }
}

impl Drawable for FixText {
    fn width(&self) -> Range {
        let center = get_text_center(self.text.as_str(), None, self.font, 1.0, 0.0);
        let x = center.x * 2.0;
        Range {
            min: x,
            max: Some(x),
        }
    }

    fn height(&self) -> Range {
        let center = get_text_center(self.text.as_str(), None, self.font, 1.0, 0.0);
        let y = center.y * -2.0;
        Range {
            min: y,
            max: Some(y),
        }
    }

    fn draw(&mut self, x: f32, y: f32, _: f32, _: f32) {
        let center = get_text_center(self.text.as_str(), None, self.font, 1.0, 0.0);
        draw_text(
            self.text.as_str(),
            x,
            y + center.y * -2.0,
            f32::from(self.font),
            self.color,
        )
    }
}
