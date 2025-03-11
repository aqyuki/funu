use crate::engine::{event, render, scene};

use super::{bullet, stage};

const CHARACTER_NORMAL_SPEED: i32 = 10;
const CHARACTER_SLOW_ATTENUATION: f32 = 0.5;
const CHARACTER_SHOT_DELAY_FRAME: i32 = 3;

#[derive(Clone, Copy)]
pub struct Character {
    position: (i32, i32),
    speed: i32,
    slow_attenuation: f32,
    before_move: (i32, i32),
    shot_delay_frame: i32,
    frames_since_last_shot: i32,
}

impl Character {
    pub fn new(position: (i32, i32)) -> Character {
        Character {
            position,
            speed: CHARACTER_NORMAL_SPEED,
            slow_attenuation: CHARACTER_SLOW_ATTENUATION,
            before_move: (0, 0),
            shot_delay_frame: CHARACTER_SHOT_DELAY_FRAME,
            frames_since_last_shot: 0,
        }
    }

    pub fn update(
        &mut self,
        event: event::Event,
        stage: &Box<dyn stage::Stage>,
        command: &mut scene::SceneCommand,
    ) {
        self.frames_since_last_shot += 1;
        if event.is_shoot && self.frames_since_last_shot >= self.shot_delay_frame {
            command.append_bullet(bullet::Bullet::new(
                (self.position.0 - 7, self.position.1 - 10),
                (0, -10),
            ));
            command.append_bullet(bullet::Bullet::new(
                (self.position.0 + 7, self.position.1 - 10),
                (0, -10),
            ));
            self.frames_since_last_shot = 0;
        }

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

        self.before_move = (dx, dy);
        self.position = match stage.is_inside_stage(x, y) {
            true => (x, y),
            false => stage.fix_position(x, y),
        };
    }
}

impl render::Drawable for Character {
    fn get_render_info(&self) -> render::Texture {
        render::Texture::Rectangle {
            center: self.position,
            size: (25, 25),
            color: (0, 0, 0),
        }
    }
}
