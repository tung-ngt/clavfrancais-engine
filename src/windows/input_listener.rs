use crate::input_listener::{Event, ListenForEvent, Listener};
use crate::keys::Key;
use crate::windows::keys_converter::KeyConverter;
use lazy_static::lazy_static;
use std::char;
use std::ptr::null_mut;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, GetKeyboardLayout, GetKeyboardState, ToUnicodeEx, HKL, VK_PACKET, VK_SHIFT,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, PeekMessageW, SetWindowsHookExW, UnhookWindowsHookEx, WaitMessage, HC_ACTION,
    HHOOK, KBDLLHOOKSTRUCT, LLKHF_INJECTED, PEEK_MESSAGE_REMOVE_TYPE, WH_KEYBOARD_LL, WH_MOUSE_LL,
    WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP,
    WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

lazy_static! {
    static ref IS_LISTENING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

static mut SENDER: Option<Sender<Event>> = None;
static mut KEYBOARD_STATE: Option<WindowsKeyboardListenerState> = None;

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

pub struct WindowsListener;

impl WindowsListener {
    unsafe fn get_unicode_char(code: u32, scan_code: u32) -> Option<char> {
        let Some(keyboard_state) = &mut KEYBOARD_STATE else {
            return None;
        };

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
            1 => char::decode_utf16(buff).next().unwrap().ok(),
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

    unsafe fn process_event(code: i32, param: WPARAM, lpdata: LPARAM) {
        if code as u32 != HC_ACTION {
            return;
        }
        match param.0 as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                let keyboard_struct = *(lpdata.0 as *const KBDLLHOOKSTRUCT);
                let virtual_key_code = keyboard_struct.vkCode;
                let scan_code = keyboard_struct.scanCode;

                let flags = keyboard_struct.flags;

                let is_injected = flags & LLKHF_INJECTED;
                if is_injected.0 != 0 {
                    return;
                }

                let has_unicode_flag = virtual_key_code == VK_PACKET.0 as u32;

                let unicode_char = if has_unicode_flag {
                    char::from_u32(scan_code)
                } else {
                    Self::get_unicode_char(virtual_key_code, scan_code)
                };

                let key = Key::from_virtual_key_code(virtual_key_code);

                if let Some(sender) = &SENDER {
                    let _ = sender.send(Event::Key { unicode_char, key });
                };
            }
            WM_KEYUP | WM_SYSKEYUP => {
                // println!("keyup");
            }
            WM_RBUTTONDOWN | WM_RBUTTONUP | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONDOWN
            | WM_MBUTTONUP => {
                if let Some(sender) = &SENDER {
                    let _ = sender.send(Event::Mouse);
                }
            }
            _ => (),
        }
    }

    unsafe extern "system" fn raw_callback(code: i32, param: WPARAM, lpdata: LPARAM) -> LRESULT {
        Self::process_event(code, param, lpdata);
        CallNextHookEx(HHOOK(null_mut()), code, param, lpdata)
    }
}

impl Listener for WindowsListener {
    fn start_listening(sender: Sender<Event>, listen_for_envent: ListenForEvent) -> JoinHandle<()> {
        {
            let mut is_listening = IS_LISTENING.lock().unwrap();
            *is_listening = true
        }
        unsafe {
            SENDER = Some(sender);
            KEYBOARD_STATE = Some(WindowsKeyboardListenerState::default());
        }

        thread::spawn(move || unsafe {
            let keyboard_hook = match listen_for_envent {
                ListenForEvent::Key | ListenForEvent::MouseAndKey => Some(
                    SetWindowsHookExW(
                        WH_KEYBOARD_LL,
                        Some(Self::raw_callback),
                        HINSTANCE(null_mut()),
                        0,
                    )
                    .unwrap(),
                ),
                _ => None,
            };

            let mouse_hook = match listen_for_envent {
                ListenForEvent::Mouse | ListenForEvent::MouseAndKey => Some(
                    SetWindowsHookExW(
                        WH_MOUSE_LL,
                        Some(Self::raw_callback),
                        HINSTANCE(null_mut()),
                        0,
                    )
                    .unwrap(),
                ),
                _ => None,
            };

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

            if let Some(mouse_hook) = mouse_hook {
                UnhookWindowsHookEx(mouse_hook).unwrap();
            }

            if let Some(keyboard_hook) = keyboard_hook {
                UnhookWindowsHookEx(keyboard_hook).unwrap();
            }

            SENDER = None;
        })
    }

    fn stop_listening() {
        let mut is_listening = IS_LISTENING.lock().unwrap();
        *is_listening = false;
    }
}
