mod engine;

use engine::{event, meta};
use sdl2::{event::Event, pixels::Color};

// windowのタイトル
const WINDOW_TITLE: &str = "STG Engine";

// 画面サイズ
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 500;

// FPS上限
const FPS_LIMIT: u32 = 60;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    // FPSの制御用構造体
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(FPS_LIMIT).unwrap();

    // 画面の初期化
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // 初期描画
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.present();

    // ゲームオブジェクトが必要とするメタ情報をまとめた構造体
    let meta = meta::Meta {
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
    };

    // キャラクターの情報を管理する構造体
    let mut character = engine::character::Character::new(
        meta.get_window_width() as i32 / 2,
        meta.get_window_height() as i32 / 2,
    );

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

        // キーボードの入力をイベントに変換
        let keyboard_state = event_pump.keyboard_state();
        let event = event::new_event(&meta, keyboard_state);

        // キャラクターの更新処理を実行
        character.update(event);

        // 描画処理
        //
        // 背景
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // キャラクター (を模した長方形)
        character.draw(&mut canvas);

        // 描画
        canvas.present();

        // 1/60 秒よりも早く処理が終わった場合は待機
        fps_manager.delay();
    }
}
