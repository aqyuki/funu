use crate::engine::render;

pub trait Stage: render::Drawable {
    fn initial_position(&self) -> (i32, i32) {
        (0, 0)
    }
    fn is_inside_stage(&self, x: i32, y: i32) -> bool;
    fn fix_position(&self, x: i32, y: i32) -> (i32, i32);
}

/// SimpleStageは、複雑なギミック等を持たないオーソドックスなステージです。
///
/// このステージは両端に壁(不可視)があり、その範囲内でプレイヤーは移動できます。
pub struct SimpleStage {
    width: u32,
    height: u32,
}

impl SimpleStage {
    pub fn new(width: u32, height: u32) -> SimpleStage {
        SimpleStage { width, height }
    }
}

impl Stage for SimpleStage {
    fn initial_position(&self) -> (i32, i32) {
        (self.width as i32 / 2, self.height as i32 / 2)
    }

    fn is_inside_stage(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn fix_position(&self, x: i32, y: i32) -> (i32, i32) {
        if self.is_inside_stage(x, y) {
            return (x, y);
        }

        let x = match x {
            x if x < 0 => 0,
            x if x >= self.width as i32 => self.width as i32 - 1,
            _ => x,
        };

        let y = match y {
            y if y < 0 => 0,
            y if y >= self.height as i32 => self.height as i32 - 1,
            _ => y,
        };

        (x, y)
    }
}

impl render::Drawable for SimpleStage {
    fn get_render_info(&self) -> render::Texture {
        render::Texture::Background {
            color: (255, 255, 255),
        }
    }
}
