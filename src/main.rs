use std::f32;

use sdl2::{event::Event, pixels::Color};

struct WindowState {
    pub width: u32,
    pub height: u32,
}

impl WindowState {
    pub fn new(width: u32, height: u32) -> Self {
        WindowState { width, height }
    }

    pub fn from_tupple((width, height): (u32, u32)) -> Self {
        WindowState::new(width, height)
    }
}

const CHARACTER_WIDTH: u32 = 100;
const CHARACTER_HEIGHT: u32 = 100;

// キャラクターの移動速度
const CHARACTER_NORMAL_SPEED: i32 = 5;
const CHARACTER_SLOW_SPEED_RATE: f32 = 0.5;

// FPS上限
const FPS_LIMIT: u32 = 60;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Hello SLD2", 800, 500)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let window_state = WindowState::from_tupple(window.size());

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // キャラクターの座標を管理する変数
    // 初期位置は画面の中央
    // SDL2では左上が原点なため、それに合わせて座標を求めている
    let mut character_x = (window_state.width as i32 / 2) - (CHARACTER_WIDTH as i32 / 2);
    let mut character_y = (window_state.height as i32 / 2) - (CHARACTER_HEIGHT as i32 / 2);

    // FPSの制御用構造体
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(FPS_LIMIT).unwrap();

    // 初期描画
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // キーボード入力を下にキャラクターの座標を更新する
        // ← : 左に移動
        // → : 右に移動
        // ↑ : 上に移動
        // ↓ : 下に移動
        // Shift : 移動速度を下げる
        let keyboard_state = event_pump.keyboard_state();

        // 移動速度の導出
        let spped = match keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::LShift) {
            true => (CHARACTER_NORMAL_SPEED as f32 * CHARACTER_SLOW_SPEED_RATE) as i32,
            false => CHARACTER_NORMAL_SPEED,
        };

        // 左右の移動量の計算
        let diff_x = if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Left) {
            -spped
        } else if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Right) {
            spped
        } else {
            0
        };

        // 上下の移動量の計算
        let diff_y = if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Up) {
            -spped
        } else if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::Down) {
            spped
        } else {
            0
        };

        // 処理結果の反映
        //
        // キャラクターの座標を更新
        character_x += diff_x;
        character_y += diff_y;

        // 画面外に出ないようにする
        if character_x < 0 {
            character_x = 0;
        } else if character_x > window_state.width as i32 - CHARACTER_WIDTH as i32 {
            character_x = window_state.width as i32 - CHARACTER_WIDTH as i32;
        }

        if character_y < 0 {
            character_y = 0;
        } else if character_y > window_state.height as i32 - CHARACTER_HEIGHT as i32 {
            character_y = window_state.height as i32 - CHARACTER_HEIGHT as i32;
        }

        // 描画処理
        //
        // 背景
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // キャラクター (を模した長方形)
        canvas.set_draw_color(Color::BLACK);
        canvas
            .fill_rect(sdl2::rect::Rect::new(
                character_x,
                character_y,
                CHARACTER_WIDTH,
                CHARACTER_HEIGHT,
            ))
            .unwrap();

        canvas.present();

        // 1/60 秒よりも早く処理が終わった場合は待機
        fps_manager.delay();
    }
}
