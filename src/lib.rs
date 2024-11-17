pub mod char_buffer;
pub mod engine;
pub mod input_controller;
pub mod input_listener;
pub mod input_simulator;
pub mod keys;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::input_listener::WindowsListener as KeyboardListenerImpl;
#[cfg(target_os = "windows")]
pub use windows::input_simulator::WindowsInputSimulator as InputSimulatorImpl;
