use engine::core::Engine;

mod engine;

const APPLICATION_NAMA: &str = "Rust game engine";

fn main() {
    let mut engine = Engine::new(APPLICATION_NAMA);
    engine.start();
}
