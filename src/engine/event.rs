use sdl2::keyboard::{self, Scancode};

use super::meta;

pub struct Event<'event> {
    pub meta: &'event meta::Meta,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub shift: bool,
}

pub fn new_event<'event>(
    meta: &'event meta::Meta,
    state: keyboard::KeyboardState,
) -> Event<'event> {
    Event {
        meta,
        up: state.is_scancode_pressed(Scancode::Up),
        down: state.is_scancode_pressed(Scancode::Down),
        left: state.is_scancode_pressed(Scancode::Left),
        right: state.is_scancode_pressed(Scancode::Right),
        shift: state.is_scancode_pressed(Scancode::LShift),
    }
}
