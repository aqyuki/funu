fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _window = video_subsystem
        .window("Hello SLD2", 800, 500)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();
}
