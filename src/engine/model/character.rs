use crate::engine::{event, render, scene};

const CHARACTER_NORMAL_SPEED: i32 = 10;
const CHARACTER_SLOW_ATTENUATION: f32 = 0.5;

#[derive(Clone, Copy)]
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

    pub fn update(self, event: event::Event, scene: &mut scene::Scene) -> Character {
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

        let (x, y) = (self.position.0 + dx, self.position.1 + dy);

        let stage = scene.get_stage_info();
        let position = match stage.is_inside_stage(x, y) {
            true => (x, y),
            false => stage.fix_position(x, y),
        };

        Character {
            position,
            speed: self.speed,
            slow_attenuation: self.slow_attenuation,
            before_move: (dx, dy),
        }
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
