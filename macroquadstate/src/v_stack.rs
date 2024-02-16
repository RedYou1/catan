use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct VStack
where
    Self: Sized,
{
    elements: Vec<Box<dyn Drawable>>,
    width: Range,
    height: Range,
}

impl VStack {
    pub fn new(elements: Vec<Box<dyn Drawable>>) -> Self {
        let width = if elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            let max = elements
                .iter()
                .filter_map(|e| e.width().max)
                .collect::<Vec<f32>>();
            Range {
                min: elements
                    .iter()
                    .map(|e| e.width().min)
                    .max_by(|x, y| x.partial_cmp(&y).unwrap())
                    .expect("no element"),
                max: if elements.len() != max.len() {
                    None
                } else {
                    max.iter()
                        .max_by(|x, y| x.partial_cmp(&y).unwrap())
                        .copied()
                },
            }
        };
        let height = if elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            let max: Vec<f32> = elements.iter().filter_map(|e| e.height().max).collect();
            Range {
                min: elements.iter().map(|e| e.height().min).sum(),
                max: if max.len() != elements.len() {
                    None
                } else {
                    Some(max.iter().sum())
                },
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
impl Drawable for VStack {
    fn width(&self) -> Range {
        self.width.clone()
    }

    fn height(&self) -> Range {
        self.height.clone()
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
