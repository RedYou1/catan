use crate::{drawable::Drawable, range::Range};

pub struct Margin<T: Drawable> {
    element: T,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl<T: Drawable> Margin<T> {
    pub const fn new(element: T, left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            element,
            left,
            right,
            top,
            bottom,
        }
    }

    pub const fn news(element: T, size: f32) -> Self {
        Self {
            element,
            left: size,
            right: size,
            top: size,
            bottom: size,
        }
    }
}

#[profiling::all_functions]
impl<T: Drawable> Drawable for Margin<T> {
    fn width(&self) -> Range {
        let w = self.element.width();
        let s = w.min + self.left + self.right;
        let e = w.max.map(|m| m + self.left + self.right);
        Range { min: s, max: e }
    }

    fn height(&self) -> Range {
        let h = self.element.height();
        let s = h.min + self.top + self.bottom;
        let e = h.max.map(|m| m + self.top + self.bottom);
        Range { min: s, max: e }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), ()> {
        self.element.draw(
            x + self.left,
            y + self.top,
            width - self.left - self.right,
            height - self.top - self.bottom,
        )
    }
}
