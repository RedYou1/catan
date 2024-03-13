use std::ptr::addr_of;

use crate::{drawable::Drawable, range::Range, wrapper::RefWrapper};

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
    pub const fn new(data: K) -> Self {
        Self {
            data,
            to_redraw: true,
            draw: None,
        }
    }

    pub const fn need_redraw(&self) -> bool {
        self.to_redraw
    }

    pub const fn data(&self) -> &K {
        &self.data
    }

    #[profiling::function]
    pub fn mutate<Func: Fn(&mut K)>(&mut self, func: Func) {
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

    fn draw(&mut self, x: f32, y: f32, width: f32, height: f32) -> Result<(), ()> {
        if self.to_redraw {
            self.to_redraw = false;
            self.draw = Some(K::gen_draw(self));
        }
        self.draw
            .as_mut()
            .expect("can't draw state")
            .draw(x, y, width, height)
    }
}

impl<K: DrawableState<V>, V: Drawable> AsRef<K> for State<K, V> {
    fn as_ref(&self) -> &K {
        &self.data
    }
}

pub struct SubState<K: DrawableState<V> + 'static, V: Drawable + 'static> {
    state: State<K, V>,
}

impl<K: DrawableState<V> + 'static, V: Drawable + 'static> SubState<K, V> {
    pub const fn new(data: K) -> Self {
        Self {
            state: State::new(data),
        }
    }

    pub fn draw(&self) -> RefWrapper {
        RefWrapper::new(addr_of!(self.state).cast_mut())
    }
}

impl<K: DrawableState<V> + 'static, V: Drawable + 'static> AsRef<State<K, V>> for SubState<K, V> {
    fn as_ref(&self) -> &State<K, V> {
        &self.state
    }
}

impl<K: DrawableState<V> + 'static, V: Drawable + 'static> AsMut<State<K, V>> for SubState<K, V> {
    fn as_mut(&mut self) -> &mut State<K, V> {
        &mut self.state
    }
}
