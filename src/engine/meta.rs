#[derive(Debug)]
pub struct Meta {
    pub window_width: u32,
    pub window_height: u32,
}

impl Meta {
    pub fn get_window_width(&self) -> u32 {
        self.window_width
    }

    pub fn get_window_height(&self) -> u32 {
        self.window_height
    }
}
