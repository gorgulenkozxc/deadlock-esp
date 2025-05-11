#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]

use serde::{Deserialize, Serialize};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, INPUT, INPUT_KEYBOARD, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, SendInput,
    VIRTUAL_KEY,
};

#[derive(Clone, Copy, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum VirtualKeys {
    LBUTTON = 1,
    RBUTTON = 2,
    CANCEL = 3,
    MBUTTON = 4,
    XBUTTON1 = 5,
    XBUTTON2 = 6,
    BACK = 8,
    TAB = 9,
    CLEAR = 12,
    RETURN = 13,
    SHIFT = 16,
    CONTROL = 17,
    MENU = 18,
    PAUSE = 19,
    CAPITAL = 20,
    HANGUL = 21,
    JUNJA = 23,
    FINAL = 24,
    ESCAPE = 27,
    CONVERT = 28,
    NONCONVERT = 29,
    ACCEPT = 30,
    MODECHANGE = 31,
    SPACE = 32,
    PRIOR = 33,
    NEXT = 34,
    END = 35,
    HOME = 36,
    LEFT = 37,
    UP = 38,
    RIGHT = 39,
    DOWN = 40,
    SELECT = 41,
    PRINT = 42,
    EXECUTE = 43,
    SNAPSHOT = 44,
    INSERT = 45,
    DELETE = 46,
    HELP = 47,
    KEY_0 = 48,
    KEY_1 = 49,
    KEY_2 = 50,
    KEY_3 = 51,
    KEY_4 = 52,
    KEY_5 = 53,
    KEY_6 = 54,
    KEY_7 = 55,
    KEY_8 = 56,
    KEY_9 = 57,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    LWIN = 91,
    RWIN = 92,
    APPS = 93,
    SLEEP = 95,
    NUMPAD0 = 96,
    NUMPAD1 = 97,
    NUMPAD2 = 98,
    NUMPAD3 = 99,
    NUMPAD4 = 100,
    NUMPAD5 = 101,
    NUMPAD6 = 102,
    NUMPAD7 = 103,
    NUMPAD8 = 104,
    NUMPAD9 = 105,
    MULTIPLY = 106,
    ADD = 107,
    SEPARATOR = 108,
    SUBTRACT = 109,
    DECIMAL = 110,
    DIVIDE = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F13 = 124,
    F14 = 125,
    F15 = 126,
    F16 = 127,
    F17 = 128,
    F18 = 129,
    F19 = 130,
    F20 = 131,
    F21 = 132,
    F22 = 133,
    F23 = 134,
    F24 = 135,
    NUMLOCK = 144,
    SCROLL = 145,
    LSHIFT = 160,
    RSHIFT = 161,
    LCONTROL = 162,
    RCONTROL = 163,
    LMENU = 164,
    RMENU = 165,
    BROWSER_BACK = 166,
    BROWSER_FORWARD = 167,
    BROWSER_REFRESH = 168,
    BROWSER_STOP = 169,
    BROWSER_SEARCH = 170,
    BROWSER_FAVORITES = 171,
    BROWSER_HOME = 172,
    VOLUME_MUTE = 173,
    VOLUME_DOWN = 174,
    VOLUME_UP = 175,
    MEDIA_NEXT_TRACK = 176,
    MEDIA_PREV_TRACK = 177,
    MEDIA_STOP = 178,
    MEDIA_PLAY_PAUSE = 179,
    LAUNCH_MAIL = 180,
    LAUNCH_MEDIA_SELECT = 181,
    LAUNCH_APP1 = 182,
    LAUNCH_APP2 = 183,
    OEM_1 = 186,
    OEM_PLUS = 187,
    OEM_COMMA = 188,
    OEM_MINUS = 189,
    OEM_PERIOD = 190,
    OEM_2 = 191,
    OEM_3 = 192,
    OEM_4 = 219,
    OEM_5 = 220,
    OEM_6 = 221,
    OEM_7 = 222,
    OEM_8 = 223,
    OEM_102 = 226,
    PROCESSKEY = 229,
    PACKET = 231,
    ATTN = 246,
    CRSEL = 247,
    EXSEL = 248,
    EREOF = 249,
    PLAY = 250,
    ZOOM = 251,
    NONAME = 252,
    PA1 = 253,
    OEM_CLEAR = 254,
}

impl VirtualKeys {
    pub fn get_keys() -> Vec<VirtualKeys> {
        vec![
            VirtualKeys::LBUTTON,
            VirtualKeys::RBUTTON,
            VirtualKeys::CANCEL,
            VirtualKeys::MBUTTON,
            VirtualKeys::XBUTTON1,
            VirtualKeys::XBUTTON2,
            VirtualKeys::BACK,
            VirtualKeys::TAB,
            VirtualKeys::CLEAR,
            VirtualKeys::RETURN,
            VirtualKeys::SHIFT,
            VirtualKeys::CONTROL,
            VirtualKeys::MENU,
            VirtualKeys::PAUSE,
            VirtualKeys::CAPITAL,
            VirtualKeys::HANGUL,
            VirtualKeys::JUNJA,
            VirtualKeys::FINAL,
            VirtualKeys::ESCAPE,
            VirtualKeys::CONVERT,
            VirtualKeys::NONCONVERT,
            VirtualKeys::ACCEPT,
            VirtualKeys::MODECHANGE,
            VirtualKeys::SPACE,
            VirtualKeys::PRIOR,
            VirtualKeys::NEXT,
            VirtualKeys::END,
            VirtualKeys::HOME,
            VirtualKeys::LEFT,
            VirtualKeys::UP,
            VirtualKeys::RIGHT,
            VirtualKeys::DOWN,
            VirtualKeys::SELECT,
            VirtualKeys::PRINT,
            VirtualKeys::EXECUTE,
            VirtualKeys::SNAPSHOT,
            VirtualKeys::INSERT,
            VirtualKeys::DELETE,
            VirtualKeys::HELP,
            VirtualKeys::KEY_0,
            VirtualKeys::KEY_1,
            VirtualKeys::KEY_2,
            VirtualKeys::KEY_3,
            VirtualKeys::KEY_4,
            VirtualKeys::KEY_5,
            VirtualKeys::KEY_6,
            VirtualKeys::KEY_7,
            VirtualKeys::KEY_8,
            VirtualKeys::KEY_9,
            VirtualKeys::KEY_A,
            VirtualKeys::KEY_B,
            VirtualKeys::KEY_C,
            VirtualKeys::KEY_D,
            VirtualKeys::KEY_E,
            VirtualKeys::KEY_F,
            VirtualKeys::KEY_G,
            VirtualKeys::KEY_H,
            VirtualKeys::KEY_I,
            VirtualKeys::KEY_J,
            VirtualKeys::KEY_K,
            VirtualKeys::KEY_L,
            VirtualKeys::KEY_M,
            VirtualKeys::KEY_N,
            VirtualKeys::KEY_O,
            VirtualKeys::KEY_P,
            VirtualKeys::KEY_Q,
            VirtualKeys::KEY_R,
            VirtualKeys::KEY_S,
            VirtualKeys::KEY_T,
            VirtualKeys::KEY_U,
            VirtualKeys::KEY_V,
            VirtualKeys::KEY_W,
            VirtualKeys::KEY_X,
            VirtualKeys::KEY_Y,
            VirtualKeys::KEY_Z,
            VirtualKeys::LWIN,
            VirtualKeys::RWIN,
            VirtualKeys::APPS,
            VirtualKeys::SLEEP,
            VirtualKeys::NUMPAD0,
            VirtualKeys::NUMPAD1,
            VirtualKeys::NUMPAD2,
            VirtualKeys::NUMPAD3,
            VirtualKeys::NUMPAD4,
            VirtualKeys::NUMPAD5,
            VirtualKeys::NUMPAD6,
            VirtualKeys::NUMPAD7,
            VirtualKeys::NUMPAD8,
            VirtualKeys::NUMPAD9,
            VirtualKeys::MULTIPLY,
            VirtualKeys::ADD,
            VirtualKeys::SEPARATOR,
            VirtualKeys::SUBTRACT,
            VirtualKeys::DECIMAL,
            VirtualKeys::DIVIDE,
            VirtualKeys::F1,
            VirtualKeys::F2,
            VirtualKeys::F3,
            VirtualKeys::F4,
            VirtualKeys::F5,
            VirtualKeys::F6,
            VirtualKeys::F7,
            VirtualKeys::F8,
            VirtualKeys::F9,
            VirtualKeys::F10,
            VirtualKeys::F11,
            VirtualKeys::F12,
            VirtualKeys::F13,
            VirtualKeys::F14,
            VirtualKeys::F15,
            VirtualKeys::F16,
            VirtualKeys::F17,
            VirtualKeys::F18,
            VirtualKeys::F19,
            VirtualKeys::F20,
            VirtualKeys::F21,
            VirtualKeys::F22,
            VirtualKeys::F23,
            VirtualKeys::F24,
            VirtualKeys::NUMLOCK,
            VirtualKeys::SCROLL,
            VirtualKeys::LSHIFT,
            VirtualKeys::RSHIFT,
            VirtualKeys::LCONTROL,
            VirtualKeys::RCONTROL,
            VirtualKeys::LMENU,
            VirtualKeys::RMENU,
            VirtualKeys::BROWSER_BACK,
            VirtualKeys::BROWSER_FORWARD,
            VirtualKeys::BROWSER_REFRESH,
            VirtualKeys::BROWSER_STOP,
            VirtualKeys::BROWSER_SEARCH,
            VirtualKeys::BROWSER_FAVORITES,
            VirtualKeys::BROWSER_HOME,
            VirtualKeys::VOLUME_MUTE,
            VirtualKeys::VOLUME_DOWN,
            VirtualKeys::VOLUME_UP,
            VirtualKeys::MEDIA_NEXT_TRACK,
            VirtualKeys::MEDIA_PREV_TRACK,
            VirtualKeys::MEDIA_STOP,
            VirtualKeys::MEDIA_PLAY_PAUSE,
            VirtualKeys::LAUNCH_MAIL,
            VirtualKeys::LAUNCH_MEDIA_SELECT,
            VirtualKeys::LAUNCH_APP1,
            VirtualKeys::LAUNCH_APP2,
            VirtualKeys::OEM_1,
            VirtualKeys::OEM_PLUS,
            VirtualKeys::OEM_COMMA,
            VirtualKeys::OEM_MINUS,
            VirtualKeys::OEM_PERIOD,
            VirtualKeys::OEM_2,
            VirtualKeys::OEM_3,
            VirtualKeys::OEM_4,
            VirtualKeys::OEM_5,
            VirtualKeys::OEM_6,
            VirtualKeys::OEM_7,
            VirtualKeys::OEM_8,
            VirtualKeys::OEM_102,
            VirtualKeys::PROCESSKEY,
            VirtualKeys::PACKET,
            VirtualKeys::ATTN,
            VirtualKeys::CRSEL,
            VirtualKeys::EXSEL,
            VirtualKeys::EREOF,
            VirtualKeys::PLAY,
            VirtualKeys::ZOOM,
            VirtualKeys::NONAME,
            VirtualKeys::PA1,
            VirtualKeys::OEM_CLEAR,
        ]
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum KeyState {
    None,
    Pressed,
    Down,
    Released,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Key {
    pub state: KeyState,
    pub code: i32,
}

impl Key {
    pub fn new(vk_code: i32) -> Self {
        Self {
            state: KeyState::None,
            code: vk_code,
        }
    }

    pub fn update(&mut self) {
        unsafe {
            let down = GetAsyncKeyState(self.code) < 0;
            if down {
                match self.state {
                    KeyState::None => self.state = KeyState::Pressed,
                    KeyState::Pressed => self.state = KeyState::Down,
                    _ => (),
                }
            } else {
                match self.state {
                    KeyState::None => (),
                    KeyState::Pressed => self.state = KeyState::Released,
                    KeyState::Down => self.state = KeyState::Released,
                    KeyState::Released => self.state = KeyState::None,
                }
            }
        }
    }
}

pub fn send_key(vk_code: VirtualKeys) {
    let mut input_down = INPUT {
        r#type: INPUT_KEYBOARD,
        ..INPUT::default()
    };
    input_down.Anonymous.ki.wVk = VIRTUAL_KEY(vk_code as u16);
    input_down.Anonymous.ki.dwFlags = KEYEVENTF_EXTENDEDKEY;

    let mut input_up = INPUT {
        r#type: INPUT_KEYBOARD,
        ..INPUT::default()
    };
    input_up.Anonymous.ki.wVk = VIRTUAL_KEY(vk_code as u16);
    input_up.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

    let inputs = [input_down, input_up];
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn send_key_thread(vk_code: VirtualKeys) {
    let mut input_down = INPUT {
        r#type: INPUT_KEYBOARD,
        ..INPUT::default()
    };
    input_down.Anonymous.ki.wVk = VIRTUAL_KEY(vk_code as u16);
    input_down.Anonymous.ki.dwFlags = KEYEVENTF_EXTENDEDKEY;

    let mut input_up = INPUT {
        r#type: INPUT_KEYBOARD,
        ..INPUT::default()
    };
    input_up.Anonymous.ki.wVk = VIRTUAL_KEY(vk_code as u16);
    input_up.Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

    let inputs_down = [input_down];
    let inputs_up = [input_up];

    std::thread::spawn(move || {
        unsafe {
            SendInput(&inputs_down, std::mem::size_of::<INPUT>() as i32);
            std::thread::sleep(std::time::Duration::from_millis(50));
            SendInput(&inputs_up, std::mem::size_of::<INPUT>() as i32);
        };
    });
}
