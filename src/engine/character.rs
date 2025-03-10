use sdl2::{render, video};

use super::{engine::GameObject, event};

// キャラクターの移動速度
const CHARACTER_NORMAL_SPEED: i32 = 10;
const CHARACTER_SLOW_SPEED_RATE: f32 = 0.5;

// キャラクターのサイズ
const CHARACTER_WIDTH: u32 = 30;
const CHARACTER_HEIGHT: u32 = 30;

pub struct Character {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
    before_dx: i32,
    before_dy: i32,
    speed: i32,
    slow_rate: f32,
}

impl Character {
    pub fn new(x: i32, y: i32) -> Self {
        Character {
            width: CHARACTER_WIDTH,
            height: CHARACTER_HEIGHT,
            x,
            y,
            before_dx: 0,
            before_dy: 0,
            speed: CHARACTER_NORMAL_SPEED,
            slow_rate: CHARACTER_SLOW_SPEED_RATE,
        }
    }
}

impl GameObject for Character {
    fn update(&mut self, event: event::Event) {
        // キャラクターの移動速度を決定
        let speed = match event.shift {
            true => (self.speed as f32 * self.slow_rate) as i32,
            false => self.speed,
        };

        // X方向の移動量を決定
        let dx = match (event.right, event.left) {
            (true, false) => speed,
            (false, true) => -speed,
            (true, true) => self.before_dx,
            _ => 0,
        };

        // Y方向の移動量を決定
        let dy = match (event.down, event.up) {
            (true, false) => speed,
            (false, true) => -speed,
            (true, true) => self.before_dy,
            _ => 0,
        };

        // 移動量を記憶
        //
        // 左右・上下同時押しの場合に、急に止まると違和感があるため、先に押されていたほうを優先する
        self.before_dx = dx;
        self.before_dy = dy;

        // 座標の更新
        self.x += dx;
        self.y += dy;

        // 画面外に出ないようにする
        self.x = if self.x < 0 {
            0
        } else if self.x > event.meta.get_window_width() as i32 {
            event.meta.get_window_width() as i32
        } else {
            self.x
        };

        self.y = if self.y < 0 {
            0
        } else if self.y > event.meta.get_window_height() as i32 {
            event.meta.get_window_height() as i32
        } else {
            self.y
        };
    }

    fn draw(&self, canvas: &mut render::Canvas<video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas
            .fill_rect(sdl2::rect::Rect::new(
                self.x - (self.width as i32 / 2),
                self.y - (self.height as i32 / 2),
                self.width,
                self.height,
            ))
            .unwrap();
    }
}
