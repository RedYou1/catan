use crate::{drawable::Drawable, range::Range};

pub struct CenterV<T: Drawable> {
    element: T,
}

impl<T: Drawable> CenterV<T> {
    pub const fn new(element: T) -> Self {
        Self { element }
    }
}

#[profiling::all_functions]
impl<T: Drawable> Drawable for CenterV<T> {
    fn width(&self) -> Range {
        self.element.width()
    }

    fn height(&self) -> Range {
        Range {
            min: self.element.height().min,
            max: None,
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), ()> {
        let h = self.element.height();
        let nheight = if height > h.max.unwrap_or(f32::MAX) {
            h.max.unwrap_or(f32::MAX)
        } else {
            height
        };
        self.element
            .draw(x, y + (height - nheight) / 2.0, width, nheight)
    }
}

pub struct CenterH<T: Drawable> {
    element: T,
}

impl<T: Drawable> CenterH<T> {
    pub const fn new(element: T) -> Self {
        Self { element }
    }
}

#[profiling::all_functions]
impl<T: Drawable> Drawable for CenterH<T> {
    fn width(&self) -> Range {
        Range {
            min: self.element.width().min,
            max: None,
        }
    }

    fn height(&self) -> Range {
        self.element.height()
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), ()> {
        let w = self.element.width();
        let nwidth = if width > w.max.unwrap_or(f32::MAX) {
            w.max.unwrap_or(f32::MAX)
        } else {
            width
        };
        self.element
            .draw(x + (width - nwidth) / 2.0, y, nwidth, height)
    }
}

pub struct Center<T: Drawable> {
    element: T,
}

impl<T: Drawable> Center<T> {
    pub const fn new(element: T) -> Self {
        Self { element }
    }
}

#[profiling::all_functions]
impl<T: Drawable> Drawable for Center<T> {
    fn width(&self) -> Range {
        Range {
            min: self.element.width().min,
            max: None,
        }
    }

    fn height(&self) -> Range {
        Range {
            min: self.element.height().min,
            max: None,
        }
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), ()> {
        let w = self.element.width();
        let nwidth = if width > w.max.unwrap_or(f32::MAX) {
            w.max.unwrap_or(f32::MAX)
        } else {
            width
        };
        let h = self.element.height();
        let nheight = if height > h.max.unwrap_or(f32::MAX) {
            h.max.unwrap_or(f32::MAX)
        } else {
            height
        };
        self.element.draw(
            x + (width - nwidth) / 2.0,
            y + (height - nheight) / 2.0,
            nwidth,
            nheight,
        )
    }
}
