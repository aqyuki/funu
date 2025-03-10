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
    }
}
