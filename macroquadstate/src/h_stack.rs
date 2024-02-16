use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct HStack
where
    Self: Sized,
{
    elements: Vec<Box<dyn Drawable>>,
    width: Range,
    height: Range,
}

impl HStack {
    pub fn new(elements: Vec<Box<dyn Drawable>>) -> Self {
        let width = if elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            let max: Vec<f32> = elements.iter().filter_map(|e| e.width().max).collect();
            Range {
                min: elements.iter().map(|e| e.width().min).sum(),
                max: if max.len() != elements.len() {
                    None
                } else {
                    Some(max.iter().sum())
                },
            }
        };
        let height = if elements.is_empty() {
            Range {
                min: 0.0,
                max: Some(0.0),
            }
        } else {
            let max = elements
                .iter()
                .filter_map(|e| e.height().max)
                .collect::<Vec<f32>>();
            Range {
                min: elements
                    .iter()
                    .map(|e| e.height().min)
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
        Self {
            elements,
            width,
            height,
        }
    }
}

#[profiling::all_functions]
impl Drawable for HStack {
    fn width(&self) -> Range {
        self.width.clone()
    }

    fn height(&self) -> Range {
        self.height.clone()
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
