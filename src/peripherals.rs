use std::mem::{size_of, transmute};

use winapi::ctypes::c_int;
use winapi::um::winuser::*;

pub fn is_mouse_clicked() -> bool {
    let res = unsafe { GetKeyState(VK_LBUTTON) };

    res & (1 << 15) != 0
}

pub fn unclick_mouse() {
    let mut input = INPUT {
        type_: INPUT_MOUSE,
        u: unsafe {
            transmute(MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_LEFTUP,
                dwExtraInfo: 0,
                time: 0,
            })
        },
    };

    unsafe { SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int) };
}
