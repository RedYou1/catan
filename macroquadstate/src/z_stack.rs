use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct ZStack
where
    Self: Sized,
{
    elements: Vec<Box<dyn Drawable>>,
    width: Range,
    height: Range,
}

impl ZStack {
    pub fn new(elements: Vec<Box<dyn Drawable>>) -> Self {
        let width = if elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            Range {
                min: elements
                    .iter()
                    .map(|e| e.width().min)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .expect("no element"),
                max: elements
                    .iter()
                    .filter_map(|e: &Box<dyn Drawable>| e.width().max)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap()),
            }
        };
        let height = if elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            Range {
                min: elements
                    .iter()
                    .map(|e| e.height().min)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .expect("no element"),
                max: elements
                    .iter()
                    .filter_map(|e: &Box<dyn Drawable>| e.height().max)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap()),
            }
        };
        Self {
            elements,
            width,
            height,
        }
    }
}

#[profiling::all_functions]
impl Drawable for ZStack {
    fn width(&self) -> Range {
        self.width.clone()
    }

    fn height(&self) -> Range {
        self.height.clone()
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
