use core::mem::size_of;

use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
        VIRTUAL_KEY,
    },
    WindowsAndMessaging::GetMessageExtraInfo,
};

use crate::input_simulator::InputSimulator;

pub struct WindowsInputSimulator;

impl InputSimulator for WindowsInputSimulator {
    fn text(content: &str) {}

    fn character(c: char) {
        unsafe {
            let input_array = [INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        time: 0,
                        wVk: VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: KEYEVENTF_UNICODE,
                        dwExtraInfo: GetMessageExtraInfo().0 as usize,
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        time: 0,
                        wVk: VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        dwExtraInfo: GetMessageExtraInfo().0 as usize,
                    },
                },
            }];
            let input_size = size_of::<INPUT>().try_into().unwrap();

            SendInput(&input_array, input_size);
        }
    }
}
