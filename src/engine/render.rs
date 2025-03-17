use sdl2::video::{GLContext, GLProfile};

pub struct Render {
    window: sdl2::video::Window,
    _ctx: GLContext,
}

impl Render {
    pub fn new(app_name: &str, sdl_context: &sdl2::Sdl) -> Render {
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window(app_name, 800, 600)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Render { window, _ctx: ctx }
    }

    pub fn window_size(&self) -> (u32, u32) {
        self.window.size()
    }

    pub fn render(&mut self, texture: Texture) {
        match texture {
            Texture::Background { color } => unsafe {
                gl::ClearColor(color.0 as f32, color.1 as f32, color.2 as f32, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            },
            Texture::Rectangle {
                center,
                size,
                color,
            } => {}
            Texture::None => (),
        };
    }

    pub fn apply(&mut self) {
        self.window.gl_swap_window();
    }
}

pub trait Drawable {
    fn get_render_info(&self) -> Texture {
        Texture::None
    }
}

pub enum Texture {
    Background {
        color: (u8, u8, u8), // RGB
    },
    Rectangle {
        center: (i32, i32),  // x, y
        size: (u32, u32),    // width, height
        color: (u8, u8, u8), // RGB
    },
    None,
}
