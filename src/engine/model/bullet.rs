use crate::engine::render;

use super::stage;

#[derive(Clone, Copy)]
pub struct Bullet {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
    pub is_outside: bool,
}

impl Bullet {
    pub fn new(position: (i32, i32), velocity: (i32, i32)) -> Bullet {
        Bullet {
            position,
            velocity,
            is_outside: false,
        }
    }

    pub fn update(&mut self, stage: &Box<dyn stage::Stage>) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        self.is_outside = !stage.is_inside_stage(self.position.0, self.position.1);
    }
}

impl render::Drawable for Bullet {
    fn get_render_info(&self) -> render::Texture {
        render::Texture::Rectangle {
            center: self.position,
            size: (10, 10),
            color: (255, 0, 0),
        }
    }
}
