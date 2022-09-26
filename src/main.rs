use std::{
    mem::{size_of, transmute},
    time::Instant,
};

use winapi::ctypes::c_int;
use winapi::um::winuser::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
struct Args {
    /// Name of the weapon (either Vandal or Phantom) to use
    #[clap(short, long, value_parser)]
    weapon_name: String,

    /// Number of bullets to shoot per burst
    #[clap(short, long, value_parser, default_value_t = 2)]
    num_bullets: u8,
}

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
    let args = Args::parse();

    let fire_rate = match args.weapon_name.trim().to_lowercase().as_str() {
        "vandal" => 9.75f32,
        "phantom" => 11.0f32,
        _ => todo!(),
    };
    let num_bullets = args.num_bullets;

    let sleep_time = (num_bullets as f32 / fire_rate) * 0.75;

    loop {
        let start = Instant::now();
        let mut mouse_down = is_mouse_clicked();

        'mouse: while mouse_down {
            let time_passed = start.elapsed();
            if time_passed.as_secs_f32() >= sleep_time {
                unclick_mouse();
                break 'mouse;
            }

            mouse_down = is_mouse_clicked();
        }
    }
}
