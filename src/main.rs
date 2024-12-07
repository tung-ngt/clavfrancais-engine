use std::{sync::mpsc, thread};

use clavfrancais_engine::{
    char_buffer::StackSizedCharBuffer, engine::Engine, input_controller::setup_key_combination_map,
};

fn main() {
    let mut is_french = false;

    let (shortcut_sender, shortcut_receiver) = mpsc::channel::<()>();
    Engine::set_toggle_channel(shortcut_sender);

    loop {
        let result = shortcut_receiver.recv();
        if result.is_err() {
            break;
        }
        if is_french {
            Engine::stop();
            is_french = false;
            println!("english")
        } else {
            let _ = thread::spawn(|| {
                Engine::start(
                    setup_key_combination_map(),
                    StackSizedCharBuffer::<30>::default(),
                );
            });
            is_french = true;
            println!("french");
        }
    }
}
