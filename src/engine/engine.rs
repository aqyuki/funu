use sdl2::{render, video};

use crate::event;

pub trait GameObject {
    fn update(&mut self, _event: event::Event) {}
    fn draw(&self, _canvas: &mut render::Canvas<video::Window>) {}
}
