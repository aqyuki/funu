use crate::engine::{event, render};

use super::stage;

const CHARACTER_NORMAL_SPEED: i32 = 10;
const CHARACTER_SLOW_ATTENUATION: f32 = 0.5;

pub struct Character {
    position: (i32, i32),
    speed: i32,
    slow_attenuation: f32,
    before_move: (i32, i32),
}

impl Character {
    pub fn new(position: (i32, i32)) -> Character {
        Character {
            position,
            speed: CHARACTER_NORMAL_SPEED,
            slow_attenuation: CHARACTER_SLOW_ATTENUATION,
            before_move: (0, 0),
        }
    }

    pub fn update(&mut self, event: event::Event, stage: &dyn stage::Stage) {
        let speed = match event.move_slow {
            true => (self.speed as f32 * self.slow_attenuation) as i32,
            false => self.speed,
        };

        let dx = match (event.move_left, event.move_right) {
            (true, false) => -speed,
            (false, true) => speed,
            (true, true) => self.before_move.0,
            _ => 0,
        };

        let dy = match (event.move_up, event.move_down) {
            (true, false) => -speed,
            (false, true) => speed,
            (true, true) => self.before_move.1,
            _ => 0,
        };

        self.before_move = (dx, dy);

        let (x, y) = (self.position.0 + dx, self.position.1 + dy);

        self.position = if stage.is_inside_stage(x, y) {
            (x, y)
        } else {
            stage.fix_position(x, y)
        };
    }
}

impl render::Drawable for Character {
    fn get_render_info(&self) -> render::Texture {
        render::Texture::Rectangle {
            center: self.position,
            size: (20, 20),
            color: (0, 0, 0),
        }
    }
}
