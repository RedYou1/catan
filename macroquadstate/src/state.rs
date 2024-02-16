use crate::{drawable::Drawable, range::Range};

pub trait DrawableState<T: Drawable>
where
    Self: Sized,
{
    fn state_width(&self) -> Range;
    fn state_height(&self) -> Range;
    fn gen_draw(state: &mut State<Self, T>) -> T;
}

pub struct State<K: DrawableState<V>, V: Drawable> {
    data: K,
    to_redraw: bool,
    draw: Option<V>,
}

impl<K: DrawableState<V>, V: Drawable> State<K, V> {
    pub fn new(data: K) -> Self {
        Self {
            data,
            to_redraw: true,
            draw: None,
        }
    }

    pub fn data(&self) -> &K {
        &self.data
    }

    #[profiling::function]
    pub fn mutate<Func: FnMut(&mut K)>(&mut self, func: &mut Func) {
        func(&mut self.data);
        self.to_redraw = true;
    }
}

#[profiling::all_functions]
impl<K: DrawableState<V>, V: Drawable> Drawable for State<K, V> {
    fn width(&self) -> Range {
        self.data.state_width()
    }

    fn height(&self) -> Range {
        self.data.state_height()
    }

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) {
        if self.to_redraw {
            self.to_redraw = false;
            self.draw = Some(K::gen_draw(self));
        }
        self.draw.as_mut().expect("error").draw(x, y, width, height);
    }
}
