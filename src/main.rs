use std::{
    mem::{size_of, transmute},
    thread::sleep,
    time::Duration,
};

use winapi::ctypes::c_int;
use winapi::um::winuser::*;

fn is_mouse_clicked() -> bool {
    let res = unsafe { GetKeyState(VK_LBUTTON) };

    res & (1 << 15) != 0
}

fn unclick_mouse() {
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

fn main() {
    loop {
        let mouse_down = is_mouse_clicked();
        println!(
            "The mouse is {}",
            if mouse_down { "down" } else { "not down" }
        );

        if mouse_down {
            unclick_mouse();
        }

        sleep(Duration::from_millis(100));
    }
}
