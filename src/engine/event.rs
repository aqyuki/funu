use sdl2::keyboard::{self, Scancode};

use super::meta;

#[derive(Debug, Copy, Clone)]
pub struct Event {
    pub meta: meta::Meta,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub shift: bool,
}

pub fn new_event(meta: meta::Meta, state: keyboard::KeyboardState) -> Event {
    Event {
        meta,
        up: state.is_scancode_pressed(Scancode::Up),
        down: state.is_scancode_pressed(Scancode::Down),
        left: state.is_scancode_pressed(Scancode::Left),
        right: state.is_scancode_pressed(Scancode::Right),
        shift: state.is_scancode_pressed(Scancode::LShift),
    }
}
