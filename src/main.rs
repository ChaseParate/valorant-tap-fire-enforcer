use std::{
    mem::{size_of, transmute},
    time::Instant,
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
    const FIRE_RATE: i32 = 11;
    const NUM_BULLETS: i32 = 2;

    const SLEEP_TIME: f64 = ((NUM_BULLETS as f64) / (FIRE_RATE as f64)) * 0.75;

    loop {
        let start = Instant::now();
        let mut mouse_down = is_mouse_clicked();

        'mouse: while mouse_down {
            let time_passed = start.elapsed();
            mouse_down = is_mouse_clicked();

            if time_passed.as_secs_f64() >= SLEEP_TIME {
                unclick_mouse();
                break 'mouse;
            }
        }
    }
}
