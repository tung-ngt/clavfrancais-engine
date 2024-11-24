use crate::keys::Key;
use std::{sync::mpsc::Sender, thread::JoinHandle};

#[derive(Debug)]
pub enum Event {
    Mouse,
    Key {
        unicode_char: Option<char>,
        key: Key,
    },
}

pub enum ListenForEvent {
    Mouse,
    Key,
    MouseAndKey
}

pub trait Listener {
    fn start_listening(sender: Sender<Event>, listen_for_event: ListenForEvent) -> JoinHandle<()>;
    fn stop_listening();
}
