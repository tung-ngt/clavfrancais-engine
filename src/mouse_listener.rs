use std::thread::JoinHandle;

pub type KeyboardListenerCallback = fn();
pub type MouseListenerCallback = fn();

pub struct KeyboardListenError;
pub struct MouseListenError;

pub trait KeyboardListener {

    fn add_callback( name: &'static str, callback: KeyboardListenerCallback);
    fn remove_callback( name: &'static str);
    fn start_listening() -> JoinHandle<()>;
    fn stop_listening();
}

pub trait MouseListener {
    fn listen_for_mouse(&mut self, callback: MouseListenerCallback);
    fn stop_listening(&self);
}


