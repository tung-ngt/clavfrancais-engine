use std::sync::mpsc::{self, Receiver};

use crate::{
    char_buffer::{HeapSizedCharBuffer, StackSizedCharBuffer},
    input_controller::{CombinationTarget, InputController, KeyCombinationMap},
    input_listener::{Event, ListenForEvent, Listener},
    input_simulator::InputSimulator,
    keys::{Key, CHANGE_FOCUS_KEYS},
    InputSimulatorImpl, KeyboardListenerImpl,
};

pub struct Engine {
    input_controller: InputController<HeapSizedCharBuffer>,
    open_guillmets: bool,
}

impl Engine {
    pub fn new(combination_map: KeyCombinationMap) -> Self {
        let char_buffer = HeapSizedCharBuffer::new(10); 
        Self {
            input_controller: InputController::new(combination_map, char_buffer),
            open_guillmets: true,
        }
    }

    fn handle_event(& mut self, receiver: Receiver<Event>) {
        loop {
            let event = receiver.recv().unwrap();

            match event {
                Event::Mouse => {
                    self.input_controller.clear_char_buffer();
                    continue;
                },
                Event::Key { unicode_char, key } => {
                    if CHANGE_FOCUS_KEYS.contains(&key) {
                        self.input_controller.clear_char_buffer();
                        continue;
                    }

                    if key == Key::Quote {
                        let guillements = if self.open_guillmets { '«' } else { '»' };
                        self.open_guillmets = !self.open_guillmets;

                        let _ = self.input_controller.add_char(guillements);

                        InputSimulatorImpl::backspace();
                        InputSimulatorImpl::character(guillements);
                        continue;
                    }

                    let Some(unicode_char) = unicode_char else {
                        continue;
                    };

                    println!("{:?}", key);
                    if key == Key::Backspace {
                        self.input_controller.backspace();
                        continue;
                    }

                    println!("{}", unicode_char);

                    let target = self.input_controller.add_char(unicode_char);

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
                },
            }
        }
    }

    pub fn start(mut self) -> Self {
        let (sender, receiver) = mpsc::channel::<Event>();
        KeyboardListenerImpl::start_listening(sender, ListenForEvent::MouseAndKey);
        self.handle_event(receiver);
        self
    }

    pub fn stop() {
        KeyboardListenerImpl::stop_listening();
    }
}
