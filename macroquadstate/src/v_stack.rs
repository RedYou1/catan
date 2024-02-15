use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct VStack
where
    Self: Sized,
{
    elements: Vec<Box<dyn Drawable>>,
}

impl VStack {
    pub fn new(elements: Vec<Box<dyn Drawable>>) -> Self {
        Self { elements }
    }
}

impl Drawable for VStack {
    fn width(&self) -> Range {
        if self.elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            let max = self
                .elements
                .iter()
                .filter_map(|e| e.width().max)
                .collect::<Vec<f32>>();
            Range {
                min: self
                    .elements
                    .iter()
                    .map(|e| e.width().min)
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

    fn height(&self) -> Range {
        if self.elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            let max: Vec<f32> = self
                .elements
                .iter()
                .filter_map(|e| e.height().max)
                .collect();
            Range {
                min: self.elements.iter().map(|e| e.height().min).sum(),
                max: if max.len() != self.elements.len() {
                    None
                } else {
                    Some(max.iter().sum())
                },
            }
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        let mut y = y;
        let diff = self.height();
        let diff = diff.max.unwrap_or(height) - diff.min;
        let c = self
            .elements
            .iter()
            .map(|e| e.height())
            .filter(|e| e.max.unwrap_or(f32::MAX) != e.min)
            .count();
        let diff = if c == 0 { 0.0 } else { diff / c as f32 };
        for e in self.elements.iter_mut() {
            let r = e.as_mut();
            let rw = r.width();
            let height = r.height();
            let ok = height.max.unwrap_or(f32::MAX) != height.min;
            r.draw(
                x,
                y,
                width.clamp(rw.min, rw.max.unwrap_or(f32::MAX)),
                height.min + if ok { diff } else { 0.0 },
            );
            y += height.min;
            if ok {
                y += diff;
            }
        }
    }
}
