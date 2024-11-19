use clavfrancais::{engine::Engine, input_controller::{setup_key_combination_map, InputController}};

fn main() {
    let engine = Engine::new(setup_key_combination_map());
    engine.start();
}
