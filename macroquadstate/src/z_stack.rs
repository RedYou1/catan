use crate::{drawable::Drawable, range::Range};
use macroquad::prelude::*;

pub struct ZStack<const LEN: usize>
where
    Self: Sized,
{
    elements: [Box<dyn Drawable>; LEN],
    width: Range,
    height: Range,
}

impl<const LEN: usize> ZStack<LEN> {
    #[allow(clippy::missing_panics_doc)]
    pub fn new(elements: [Box<dyn Drawable>; LEN]) -> Self {
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
                    .max_by(|x, y| x.partial_cmp(y).expect(""))
                    .expect(""),
                max: elements
                    .iter()
                    .filter_map(|e| e.width().max)
                    .max_by(|x, y| x.partial_cmp(y).expect("")),
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
                    .max_by(|x, y| x.partial_cmp(y).expect(""))
                    .expect(""),
                max: elements
                    .iter()
                    .filter_map(|e| e.height().max)
                    .max_by(|x, y| x.partial_cmp(y).expect("")),
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
impl<const LEN: usize> Drawable for ZStack<LEN> {
    fn width(&self) -> Range {
        self.width.clone()
    }

    fn height(&self) -> Range {
        self.height.clone()
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), ()> {
        for e in &mut self.elements {
            let r = e.as_mut();
            let w = r.width();
            let h = r.height();
            if let Err(()) = r.draw(
                x,
                y,
                width.clamp(w.min, w.max.unwrap_or(f32::MAX)),
                height.clamp(h.min, h.max.unwrap_or(f32::MAX)),
            ) {
                return Err(());
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! zstack {
    [$($element:expr),* $(,)?] => {
        ZStack::new([$(Box::new($element)),*])
    };
}
