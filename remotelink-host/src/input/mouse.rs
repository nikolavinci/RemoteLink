use std::mem;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn move_mouse(x: i32, y: i32) {
    unsafe {
        let mut input = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: x,
                    dy: y,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        SendInput(&[input], mem::size_of::<INPUT>() as i32);
    }
}
