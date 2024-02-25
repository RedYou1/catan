use macroquad::prelude::*;

use crate::range::Range;

pub trait Drawable {
    fn width(&self) -> Range;
    fn height(&self) -> Range;
    /// # Errors
    /// Stop to draw
    #[allow(clippy::result_unit_err)]
    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), ()>;
}
