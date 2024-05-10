use std::mem::size_of;

use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn is_mouse_clicked() -> bool {
    let state = unsafe { GetKeyState(VK_LBUTTON.0.into()) };

    state & (1 << 15) != 0
}

pub fn unclick_mouse() {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dwFlags: MOUSEEVENTF_LEFTUP,
                ..Default::default()
            },
        },
    };

    unsafe { SendInput(&[input], size_of::<INPUT>() as i32) };
}
