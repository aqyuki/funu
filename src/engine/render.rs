use sdl2::{rect::Rect, render, video};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub struct Render {
    pub canvas: render::Canvas<video::Window>,
}

impl Render {
    pub fn new(window: sdl2::video::Window) -> Render {
        let canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();
        Render { canvas }
    }

    pub fn render(&mut self, texture: Texture) {
        match texture {
            Texture::Background { color } => {
                self.canvas
                    .set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
                self.canvas.clear();
            }
            Texture::Rectangle {
                center,
                size,
                color,
            } => {
                self.canvas
                    .set_draw_color(sdl2::pixels::Color::RGB(color.0, color.1, color.2));
                self.canvas
                    .fill_rect(Rect::new(
                        center.0 - (size.0 as i32 / 2),
                        center.1 - (size.1 as i32 / 2),
                        size.0,
                        size.1,
                    ))
                    .unwrap();
            }
            Texture::None => (),
        };
    }

    pub fn apply(&mut self) {
        self.canvas.present();
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
