use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct HStack
where
    Self: Sized,
{
    elements: Vec<Box<dyn Drawable>>,
}

impl HStack {
    pub fn new(elements: Vec<Box<dyn Drawable>>) -> Self {
        Self { elements }
    }
}

impl Drawable for HStack {
    fn width(&self) -> Range {
        if self.elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            let max: Vec<f32> = self.elements.iter().filter_map(|e| e.width().max).collect();
            Range {
                min: self.elements.iter().map(|e| e.width().min).sum(),
                max: if max.len() != self.elements.len() {
                    None
                } else {
                    Some(max.iter().sum())
                },
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
            let max = self
                .elements
                .iter()
                .filter_map(|e| e.height().max)
                .collect::<Vec<f32>>();
            Range {
                min: self
                    .elements
                    .iter()
                    .map(|e| e.height().min)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .expect("no element"),
                max: if self.elements.len() != max.len() {
                    None
                } else {
                    max.iter()
                        .max_by(|x, y| x.partial_cmp(&y).unwrap())
                        .copied()
                },
            }
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let mut x = x;
        let diff = self.width();
        let diff = diff.max.unwrap_or(width) - diff.min;
        let c = self
            .elements
            .iter()
            .map(|e| e.width())
            .filter(|e| e.max.unwrap_or(f32::MAX) != e.min)
            .count();
        let diff = diff / c as f32;
        for e in self.elements.iter_mut() {
            let r = e.as_mut();
            let rh = r.height();
            let width = r.width();
            let ok = width.max.unwrap_or(f32::MAX) != width.min;
            r.draw(
                x,
                y,
                width.min + if ok { diff } else { 0.0 },
                height.clamp(rh.min, rh.max.unwrap_or(f32::MAX)),
            );
            x += width.min;
            if ok {
                x += diff;
            }
        }
    }
}
