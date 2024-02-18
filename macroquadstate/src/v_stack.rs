use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct VStack<const LEN: usize>
where
    Self: Sized,
{
    elements: [Box<dyn Drawable>; LEN],
    width: Range,
    height: Range,
}

impl<const LEN: usize> VStack<LEN> {
    #[allow(clippy::missing_panics_doc)]
    pub fn new(elements: [Box<dyn Drawable>; LEN]) -> Self {
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
                    .max_by(|x, y| x.partial_cmp(y).expect(""))
                    .expect(""),
                max: if elements.len() == max.len() {
                    max.iter()
                        .max_by(|x, y| x.partial_cmp(y).expect(""))
                        .copied()
                } else {
                    None
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
                max: if max.len() == elements.len() {
                    Some(max.iter().sum())
                } else {
                    None
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
impl<const LEN: usize> Drawable for VStack<LEN> {
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
            .filter(|e| !e.fix_sized())
            .count();
        #[allow(clippy::cast_precision_loss)]
        let diff = if c == 0 { 0.0 } else { diff / c as f32 };
        for e in &mut self.elements {
            let r = e.as_mut();
            let rw = r.width();
            let height = r.height();
            let ok = !height.fix_sized();
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

#[macro_export]
macro_rules! vstack {
    [$($element:expr),* $(,)?] => {
        VStack::new([$(Box::new($element)),*])
    };
}
