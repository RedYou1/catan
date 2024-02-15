use crate::{drawable::Drawable, range::Range};

pub struct Offset<T: Drawable> {
    x: f32,
    y: f32,
    element: T,
}

impl<T: Drawable> Offset<T> {
    pub fn new(x: f32, y: f32, element: T) -> Self {
        Self { x, y, element }
    }
}

impl<T: Drawable> Drawable for Offset<T> {
    fn width(&self) -> Range {
        let mut w = self.element.width();
        w.min += self.x;
        w.max = w.max.map(|w| w + self.x);
        w
    }

    fn height(&self) -> Range {
        let mut h = self.element.height();
        h.min += self.y;
        h.max = h.max.map(|w| w + self.y);
        h
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.element
            .draw(x + self.x, y + self.y, width - self.x, height - self.y)
    }
}
