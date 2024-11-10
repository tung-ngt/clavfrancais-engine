use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use clavfrancais::char_buffer::FixedSizeCharBuffer;
use clavfrancais::input_controller::{setup_key_combination_map, InputController};
use clavfrancais::input_simulator::InputSimulator;
use clavfrancais::keyboard_listener::{KeyEvent, KeyboardListener};
use clavfrancais::{InputSimulatorImpl, KeyboardListenerImpl};
use lazy_static::lazy_static;

lazy_static! {
    static ref input_controller: Arc<Mutex<InputController<FixedSizeCharBuffer>>> =
        Arc::new(Mutex::new(InputController::new(
            setup_key_combination_map(),
            FixedSizeCharBuffer::new(50)
        )));
}

fn something(key_event: &KeyEvent) {
    // let key = &key_event.key;
    let unicode_chars = &key_event.unicode_chars;

    if let Some(s) = unicode_chars {
        input_controller
            .lock()
            .unwrap()
            .add_char(s.chars().nth(0).unwrap());
    }
}

fn main() {

    


    // InputSimulatorImpl::character('é');
    // InputSimulatorImpl::character('é');
    // InputSimulatorImpl::character('é');
    // InputSimulatorImpl::character('é');
    // let handle = KeyboardListenerImpl::start_listening();

    // KeyboardListenerImpl::add_callback("something", something);

    // handle.join().unwrap();

    // KeyboardListenerImpl::stop_listening();
    // eee ee eww`fd`fsaasdf`as`
    // ééé  ééééé 


}

