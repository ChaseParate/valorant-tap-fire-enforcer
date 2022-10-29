use std::ffi::c_int;
use std::mem::size_of;

use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn is_mouse_clicked() -> bool {
    let res = unsafe { GetKeyState(VK_LBUTTON.0.into()) };

    res & (1 << 15) != 0
}

pub fn unclick_mouse() {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_LEFTUP,
                dwExtraInfo: 0,
                time: 0,
            },
        },
    };

    unsafe { SendInput(&[input], size_of::<INPUT>() as c_int) };
}
