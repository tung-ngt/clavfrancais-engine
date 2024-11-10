use crate::keys::Key;
use std::thread::JoinHandle;

pub struct KeyEvent {
    pub unicode_chars: Option<String>,
    pub key: Key,
}

impl KeyEvent {
    pub fn new(unicode_chars: Option<String>, key: Key) -> Self {
        KeyEvent { unicode_chars, key }
    }
}

pub type KeyboardListenerCallback = fn(&KeyEvent);
pub struct KeyboardListenError;
pub trait KeyboardListener {
    fn add_callback(name: &'static str, callback: KeyboardListenerCallback);
    fn remove_callback(name: &'static str);
    fn start_listening() -> JoinHandle<()>;
    fn stop_listening();
}
