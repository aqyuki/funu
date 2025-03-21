use super::{
    event,
    model::{
        character,
        stage::{self, Stage},
    },
    render::Render,
    scene::Scene,
};

/// Engineは各種ゲームオブジェクトやリソース・イベントを管理します。
pub struct Engine {
    app_name: String,
}

impl Engine {
    pub fn new(app_name: &str) -> Engine {
        Engine {
            app_name: app_name.to_string(),
        }
    }

    pub fn start(&mut self) {
        let sdl_context = sdl2::init().unwrap();

        // サブシステムを初期化
        let video_subsystem = sdl_context.video().unwrap();

        // ウィンドウを作成
        let window = video_subsystem
            .window(&self.app_name, 800, 600)
            .vulkan()
            .position_centered()
            .build()
            .unwrap();

        let (width, height) = window.size();

        // 画面の初期化
        let canvas = window.into_canvas().build().unwrap();
        let mut render = Render::new(canvas);

        // FPS管理用
        let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
        fps_manager.set_framerate(60).unwrap();

        // イベント処理のためのインスタンスを取得
        let mut event_pump = sdl_context.event_pump().unwrap();

        // ゲームオブジェクト
        let stage = stage::SimpleStage::new(width, height);
        let character = character::Character::new(stage.initial_position());

        let mut scene = Scene::new(character, Box::new(stage), &mut render);

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. }
                    | sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            let event = event::event_from_keyboard_input(event_pump.keyboard_state());

            scene.update(event);
            scene.render();

            fps_manager.delay();
        }
    }
}
