use crate::{drawable::Drawable, range::Range};

pub struct Wrapper {
    element: Box<dyn Drawable>,
}

impl Wrapper {
    pub fn new<T: Drawable + 'static>(element: T) -> Self {
        Self {
            element: Box::new(element) as Box<dyn Drawable>,
        }
    }
    pub fn newb(element: Box<dyn Drawable>) -> Self {
        Self { element }
    }
}

#[profiling::all_functions]
impl Drawable for Wrapper {
    fn width(&self) -> Range {
        self.element.width()
    }

    fn height(&self) -> Range {
        self.element.height()
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.element.draw(x, y, width, height);
    }
}
