use std::sync::mpsc;

use crate::{
    char_buffer::FixedSizeCharBuffer,
    input_controller::{self, CombinationTarget, InputController, KeyCombinationMap},
    input_listener::{Event, Listener},
    input_simulator::InputSimulator,
    InputSimulatorImpl, KeyboardListenerImpl,
};

pub struct Engine {
    input_controller: InputController<FixedSizeCharBuffer>,
}

impl Engine {
    pub fn new(combination_map: KeyCombinationMap) -> Self {
        let char_buffer = FixedSizeCharBuffer::new(50);
        Self {
            input_controller: InputController::new(combination_map, char_buffer),
        }
    }

    pub fn start(mut self) -> Self {
        let (sender, receiver) = mpsc::channel::<Event>();
        KeyboardListenerImpl::start_listening(sender);
        loop {
            let event = receiver.recv().unwrap();
            let Event::Key {
                unicode_chars: Some(unicode_chars),
                key: _,
            } = event
            else {
                continue;
            };

            if unicode_chars.is_empty() {
                continue;
            }

            println!("{}", unicode_chars);

            let target = self
                .input_controller
                .add_char(unicode_chars.chars().next().unwrap());

            let Some(target) = target else {
                continue;
            };
            InputSimulatorImpl::backspace();
            InputSimulatorImpl::backspace();
            
            if let CombinationTarget::Combine(a) = target {
                InputSimulatorImpl::character(*a);
                println!("{:?}", self.input_controller.char_buffer);
                continue;
            }

            if let CombinationTarget::Revert(a, b) = target {
                InputSimulatorImpl::character(*a);
                InputSimulatorImpl::character(*b);
            }
            println!("{:?}", self.input_controller.char_buffer);

        }
        self
    }

    pub fn stop() {
        KeyboardListenerImpl::stop_listening();
    }
}
