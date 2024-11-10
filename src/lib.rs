pub mod char_buffer;
pub mod input_controller;
pub mod keyboard_listener;
pub mod input_simulator;
pub mod keys;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::keyboard_listener::WindowsKeyboardListener as KeyboardListenerImpl;
#[cfg(target_os = "windows")]
pub use windows::input_simulator::WindowsInputSimulator as InputSimulatorImpl;

