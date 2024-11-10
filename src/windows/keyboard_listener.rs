use crate::keyboard_listener::{KeyEvent, KeyboardListener, KeyboardListenerCallback};
use crate::keys::Key;
use crate::windows::keys_converter::KeyConverter;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, GetKeyboardLayout, GetKeyboardState, ToUnicodeEx, HKL, VK_PACKET, VK_SHIFT
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, PeekMessageW, SetWindowsHookExW, UnhookWindowsHookEx, WaitMessage, HC_ACTION,
    HHOOK, KBDLLHOOKSTRUCT, PEEK_MESSAGE_REMOVE_TYPE, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

static mut HOOK: HHOOK = HHOOK(null_mut());
lazy_static! {
    static ref KEYBOARD_STATE: Arc<Mutex<WindowsKeyboardListenerState>> =
        Arc::new(Mutex::new(WindowsKeyboardListenerState::default()));
    static ref CALLBACK_MAP: Arc<Mutex<CallbackMap>> = Arc::new(Mutex::new(HashMap::new()));
    static ref IS_LISTENING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}
type CallbackMap = HashMap<&'static str, KeyboardListenerCallback>;

const BUFFER_LEN: i32 = 32;

struct WindowsKeyboardListenerState {
    last_code: u32,
    last_scan_code: u32,
    last_state: [u8; 256],
    last_is_dead: bool,
}

impl Default for WindowsKeyboardListenerState {
    fn default() -> Self {
        Self {
            last_code: Default::default(),
            last_scan_code: Default::default(),
            last_state: [0; 256],
            last_is_dead: Default::default(),
        }
    }
}

pub struct WindowsKeyboardListener;

impl WindowsKeyboardListener {
    unsafe fn get_unicode_char(code: u32, scan_code: u32) -> Option<String> {
        let mut keyboard_state = KEYBOARD_STATE.lock().unwrap();

        keyboard_state.last_state = Self::set_global_state();

        // let state_ptr = keyboard_state.last_state.as_mut_ptr();
        let mut buff = [0_u16; BUFFER_LEN as usize];

        let layout = GetKeyboardLayout(0);

        let len = ToUnicodeEx(
            code,
            scan_code,
            &keyboard_state.last_state,
            &mut buff,
            0,
            layout,
        );

        let mut is_dead = false;
        let result = match len {
            0 => None,
            -1 => {
                is_dead = true;
                Self::clear_keyboard_buffer(code, scan_code, layout);
                None
            }
            len if len > 0 => String::from_utf16(&buff[..len as usize]).ok(),
            _ => None,
        };

        if keyboard_state.last_code != 0 && keyboard_state.last_is_dead {
            buff = [0; 32];
            ToUnicodeEx(
                keyboard_state.last_code,
                keyboard_state.last_scan_code,
                &keyboard_state.last_state,
                &mut buff,
                0,
                layout,
            );
            keyboard_state.last_code = 0;
        } else {
            keyboard_state.last_code = code;
            keyboard_state.last_scan_code = scan_code;
            keyboard_state.last_is_dead = is_dead;
        }

        result
    }

    unsafe fn set_global_state() -> [u8; 256] {
        let mut state = [0_u8; 256];
        let _shift = GetKeyState(VK_SHIFT.0 as i32);
        GetKeyboardState(&mut state).unwrap();
        state
    }

    unsafe fn clear_keyboard_buffer(virtual_key_code: u32, scan_code: u32, layout: HKL) {
        let mut buff = [0_u16; BUFFER_LEN as usize];
        let state = [0_u8; 256];

        let mut len = -1;
        while len < 0 {
            len = ToUnicodeEx(virtual_key_code, scan_code, &state, &mut buff, 0, layout);
        }
    }

    unsafe extern "system" fn raw_callback(code: i32, param: WPARAM, lpdata: LPARAM) -> LRESULT {
        if code as u32 == HC_ACTION {
            match param.0 as u32 {
                WM_KEYDOWN | WM_SYSKEYDOWN => {
                    let keyboard_struct = *(lpdata.0 as *const KBDLLHOOKSTRUCT);
                    let virtual_key_code = keyboard_struct.vkCode;
                    let scan_code = keyboard_struct.scanCode;

                    let has_unicode_flag = virtual_key_code == VK_PACKET.0 as u32;

                    let unicode_chars = if has_unicode_flag {
                        char::from_u32(scan_code).map(|c| c.to_string())
                    } else {
                        Self::get_unicode_char(virtual_key_code, scan_code)
                    };

                    let key = Key::from_virtual_key_code(virtual_key_code);

                    let key_event = KeyEvent::new(unicode_chars, key);
                    let callback_map = CALLBACK_MAP.lock().unwrap();
                    for (_, callback) in callback_map.iter() {
                        callback(&key_event);
                    }
                }
                WM_KEYUP | WM_SYSKEYUP => {
                    // println!("keyup");
                }
                _ => (),
            }
        }
        CallNextHookEx(HOOK, code, param, lpdata)
    }
}

impl KeyboardListener for WindowsKeyboardListener {
    fn add_callback(name: &'static str, callback: KeyboardListenerCallback) {
        let mut callback_map = CALLBACK_MAP.lock().unwrap();
        callback_map.insert(name, callback);
    }

    fn remove_callback(name: &'static str) {
        let mut callback_map = CALLBACK_MAP.lock().unwrap();
        callback_map.remove(name);
    }

    fn start_listening() -> JoinHandle<()> {
        {
            let mut is_listening = IS_LISTENING.lock().unwrap();
            *is_listening = true
        }

        thread::spawn(|| unsafe {
            HOOK = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(Self::raw_callback),
                HINSTANCE(null_mut()),
                0,
            )
            .unwrap();

            loop {
                if WaitMessage().is_err() {
                    break;
                }

                let is_listenting = IS_LISTENING.lock().unwrap();
                if !*is_listenting {
                    break;
                }

                let _ = PeekMessageW(
                    null_mut(),
                    HWND(null_mut()),
                    0,
                    0,
                    PEEK_MESSAGE_REMOVE_TYPE(0),
                );
            }

            UnhookWindowsHookEx(HOOK).unwrap();
        })
    }

    fn stop_listening() {
        let mut is_listening = IS_LISTENING.lock().unwrap();
        *is_listening = false;
    }
}
