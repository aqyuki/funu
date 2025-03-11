use sdl2::keyboard::{KeyboardState, Scancode};

#[derive(Debug, Default, Clone, Copy)]
pub struct Event {
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_slow: bool,
    pub is_shoot: bool,
}

pub fn event_from_keyboard_input(keyboard_state: KeyboardState) -> Event {
    Event {
        move_up: keyboard_state.is_scancode_pressed(Scancode::Up),
        move_down: keyboard_state.is_scancode_pressed(Scancode::Down),
        move_left: keyboard_state.is_scancode_pressed(Scancode::Left),
        move_right: keyboard_state.is_scancode_pressed(Scancode::Right),
        move_slow: keyboard_state.is_scancode_pressed(Scancode::LShift),
        is_shoot: keyboard_state.is_scancode_pressed(Scancode::Z),
    }
}
