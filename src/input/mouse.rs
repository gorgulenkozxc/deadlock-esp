use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE,
};

pub fn send_move(x: i32, y: i32) {
    let mut input = INPUT {
        r#type: INPUT_MOUSE,
        ..INPUT::default()
    };
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_MOVE;
    input.Anonymous.mi.dx = x;
    input.Anonymous.mi.dy = y;
    let inputs = [input];
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn left_click() {
    let mut input = INPUT {
        r#type: INPUT_MOUSE,
        ..INPUT::default()
    };
    input.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTDOWN;

    let mut input2 = INPUT {
        r#type: INPUT_MOUSE,
        ..INPUT::default()
    };
    input2.Anonymous.mi.dwFlags = MOUSEEVENTF_LEFTUP;
    let inputs = [input2, input, input2];
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}
