use crate::keys::Key;
use std::{sync::mpsc::Sender, thread::JoinHandle};

#[derive(Debug)]
pub enum MouseKeyEvent {
    Mouse,
    Key {
        unicode_char: Option<char>,
        key: Key,
    },
}
pub trait Listener {
    fn start_mouse_key_listening(sender: Sender<MouseKeyEvent>) -> JoinHandle<()>;
    fn stop_mouse_key_listening();

    fn start_shortcut_listening(sender: Sender<()>) -> JoinHandle<()>;
    fn stop_shortcut_listening();
}
