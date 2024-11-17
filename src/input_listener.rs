use crate::keys::Key;
use std::{sync::mpsc::Sender, thread::JoinHandle};

#[derive(Debug)]
pub enum Event {
    Mouse,
    Key {
        unicode_chars: Option<String>,
        key: Key,
    },
}
pub trait Listener {
    fn start_listening(sender: Sender<Event>) -> JoinHandle<()>;
    fn stop_listening();
}
