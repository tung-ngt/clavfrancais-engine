use std::{thread, time::Duration};

use clavfrancais_engine::{char_buffer::StackSizedCharBuffer, engine::Engine, input_controller::setup_key_combination_map};

fn main() {
    let join_handle = thread::spawn(|| {
        Engine::start(setup_key_combination_map(), StackSizedCharBuffer::<30>::default());
    });

    thread::sleep(Duration::from_secs(10));

    Engine::stop();

    join_handle.join().unwrap();
}
