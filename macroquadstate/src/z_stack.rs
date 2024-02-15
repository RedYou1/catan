use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct ZStack
where
    Self: Sized,
{
    elements: Vec<Box<dyn Drawable>>,
}

impl ZStack {
    pub fn new(elements: Vec<Box<dyn Drawable>>) -> Self {
        Self { elements }
    }
}

impl Drawable for ZStack {
    fn width(&self) -> Range {
        if self.elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            Range {
                min: self
                    .elements
                    .iter()
                    .map(|e| e.width().min)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .expect("no element"),
                max: self
                    .elements
                    .iter()
                    .filter_map(|e: &Box<dyn Drawable>| e.width().max)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap()),
            }
        }
    }

    fn height(&self) -> Range {
        if self.elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            Range {
                min: self
                    .elements
                    .iter()
                    .map(|e| e.height().min)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .expect("no element"),
                max: self
                    .elements
                    .iter()
                    .filter_map(|e: &Box<dyn Drawable>| e.height().max)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap()),
            }
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        for e in self.elements.iter_mut() {
            let r = e.as_mut();
            let w = r.width();
            let h = r.height();
            r.draw(
                x,
                y,
                width.clamp(w.min, w.max.unwrap_or(f32::MAX)),
                height.clamp(h.min, h.max.unwrap_or(f32::MAX)),
            );
        }
    }
}
